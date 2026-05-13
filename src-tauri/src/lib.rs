use tauri::plugin::{Builder as PluginBuilder, TauriPlugin};
use tauri::{Manager, Wry};
use tauri_plugin_deep_link::DeepLinkExt;

/// Our own domain — navigations here stay inside the webview.
const APP_HOST: &str = "cliently-kappa.vercel.app";

fn is_external(url_str: &str) -> bool {
    // Anything that is NOT our Vercel-hosted frontend is "external".
    // Allow blob:, data:, about:, and localhost URLs as internal too.
    if url_str.starts_with("blob:") || url_str.starts_with("data:") || url_str.starts_with("about:") {
        return false;
    }
    if url_str.contains("localhost") || url_str.contains("127.0.0.1") {
        return false;
    }
    if let Ok(parsed) = url::Url::parse(url_str) {
        if let Some(host) = parsed.host_str() {
            return host != APP_HOST;
        }
    }
    false
}

fn external_browser_interceptor() -> TauriPlugin<Wry> {
    PluginBuilder::new("external_browser_interceptor")
        .js_init_script(EXTERNAL_LINK_SHIM)
        .on_navigation(|_window, url| {
            let url_str = url.as_str();

            // If the URL is our desktop auth route, open it in the system browser!
            if url_str.contains("/api/auth/google-desktop") {
                let _ = open::that(url_str);
                return false; // Cancel navigation in webview
            }

            // Open ANY external URL in the system browser instead of the webview
            if is_external(url_str) {
                let _ = open::that(url_str);
                return false; // Cancel navigation in webview
            }
            true // Allow internal navigations
        })
        .build()
}

#[tauri::command]
fn open_external_url(url: String) {
    let _ = open::that(url);
}

/// JavaScript shim injected into every page load.
const EXTERNAL_LINK_SHIM: &str = r#"
(function() {
  if (window.__CLIENTLY_SHIM_INSTALLED__) return;
  window.__CLIENTLY_SHIM_INSTALLED__ = true;

  // Helper: open URL in system browser via Tauri custom command
  function openExternal(url) {
    if (window.__TAURI_INTERNALS__) {
      window.__TAURI_INTERNALS__.invoke('open_external_url', { url: url });
    } else if (window.__TAURI__ && window.__TAURI__.core && window.__TAURI__.core.invoke) {
      window.__TAURI__.core.invoke('open_external_url', { url: url });
    } else if (window.__TAURI__ && window.__TAURI__.invoke) {
      window.__TAURI__.invoke('open_external_url', { url: url });
    }
  }

  // Check if a URL is external (not our app domain)
  function isExternal(url) {
    try {
      var parsed = new URL(url, window.location.origin);
      if (parsed.protocol === 'blob:' || parsed.protocol === 'data:') return false;
      return parsed.hostname !== 'cliently-kappa.vercel.app'
          && parsed.hostname !== 'localhost'
          && parsed.hostname !== '127.0.0.1';
    } catch(e) {
      return false;
    }
  }

  // 1. Intercept clicks on <a target="_blank"> links
  document.addEventListener('click', function(e) {
    var el = e.target;
    while (el && el.tagName !== 'A') el = el.parentElement;
    if (!el) return;

    var href = el.getAttribute('href');
    if (!href) return;

    var target = el.getAttribute('target');
    if (target === '_blank' || isExternal(href)) {
      e.preventDefault();
      e.stopPropagation();
      // Resolve relative URLs
      var fullUrl = new URL(href, window.location.origin).href;
      openExternal(fullUrl);
    }
  }, true); // useCapture so we catch before React handlers

  // 2. Override window.open to route external URLs to system browser
  var _origOpen = window.open;
  window.open = function(url, target, features) {
    if (url && (target === '_blank' || isExternal(url))) {
      var fullUrl = new URL(url, window.location.origin).href;
      openExternal(fullUrl);
      return null;
    }
    return _origOpen.call(window, url, target, features);
  };
})();
"#;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_deep_link::init())
        .plugin(external_browser_interceptor())
        .setup(|app| {
            let app_handle = app.handle().clone();


            // Process deep links triggered while the app is already running
            app.deep_link().on_open_url({
                let app_handle = app_handle.clone();
                move |event| {
                    for url in event.urls() {
                        let url_str = url.as_str();
                        if url_str.starts_with("cliently://") {
                            let new_url = url_str.replace("cliently://", "https://cliently-kappa.vercel.app/");
                            if let Some(window) = app_handle.get_webview_window("main") {
                                let _ = window.unminimize();
                                let _ = window.set_focus();
                                let script = format!("window.location.href = '{}';", new_url);
                                let _ = window.eval(&script);
                            }
                        }
                    }
                }
            });

            // Process deep links that launched the app
            if let Ok(Some(urls)) = app.deep_link().get_current() {
                for url in urls {
                    let url_str = url.as_str();
                    if url_str.starts_with("cliently://") {
                        let new_url = url_str.replace("cliently://", "https://cliently-kappa.vercel.app/");
                        if let Some(window) = app_handle.get_webview_window("main") {
                            let script = format!("window.location.href = '{}';", new_url);
                            let _ = window.eval(&script);
                        }
                    }
                }
            }

            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![open_external_url])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
