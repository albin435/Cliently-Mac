use url::Url;

fn main() {
    let url_str = "https://wgtdagasawbsqllwfppx.supabase.co/auth/v1/authorize?provider=google&redirect_to=https%3A%2F%2Fcliently-kappa.vercel.app%2Fauth%2Fcallback&access_type=offline&prompt=consent";
    let url = Url::parse(url_str).unwrap();
    
    let is_supabase_oauth = url.host_str()
        .map(|h| h.ends_with("supabase.co"))
        .unwrap_or(false)
        && url.path().contains("/auth/");

    println!("is_supabase_oauth: {}", is_supabase_oauth);

    let mut modified_url = url_str.to_string();
    if is_supabase_oauth && !url_str.contains("cliently%3A%2F%2F") {
        if let Some(redirect_pos) = modified_url.find("redirect_to=") {
            let after = &modified_url[redirect_pos..];
            if !after.contains("next%3D") {
                if let Some(amp_pos) = after.find('&') {
                    let insert_at = redirect_pos + amp_pos;
                    modified_url.insert_str(
                        insert_at,
                        "%3Fnext%3Dcliently%3A%2F%2Fauth-sync"
                    );
                } else {
                    modified_url.push_str(
                        "%3Fnext%3Dcliently%3A%2F%2Fauth-sync"
                    );
                }
            }
        }
    }

    println!("Modified URL: {}", modified_url);
}
