const REGISTRY_URL : &str = "https://registry.hub.docker.com";
const AUTH_URL : &str = "https://auth.docker.io";

pub fn pull_image(image_id: String, result_dir: String) -> Result<(), reqwest::Error> {
    // parse image + tag
    let split : Vec<&str> = image_id.split(':').collect();
    let image = split[0];
    let tag = split[1];
    
    let auth = get_auth(String::from(image))?;
    let blob = get_manifest(String::from(image), String::from(tag), auth.clone()).unwrap();
    blob_to_file(image.to_string(), blob, auth, result_dir).unwrap();

    Ok(())
}

fn get_auth(image: String) -> Result<String, reqwest::Error> {
    let body : serde_json::Value = reqwest::blocking::get(&format!("{}/token?service=registry.docker.io&scope=repository:library/{}:pull", AUTH_URL, image))?
        .json::<serde_json::Value>()?;
    
    Ok(String::from(body["token"].as_str().unwrap()))
}

fn get_manifest(image: String, tag: String, token:String) -> Result<Vec<String>, reqwest::Error> {
    let client = reqwest::blocking::Client::new();
    let body : serde_json::Value = client
        .get(&format!("{}/v2/library/{}/manifests/{}", REGISTRY_URL, image, tag))
        .bearer_auth(token.clone())
        .send()?
        .json::<serde_json::Value>()?;

    let mut result : Vec<String> = Vec::new();
    
    if let Some(fs_layers) = body["fsLayers"].as_array() {
        for elem in fs_layers {
            result.push(String::from(elem["blobSum"].as_str().unwrap()));
        }
    }

    Ok(result)
}

fn blob_to_file(image: String, blob: Vec<String>, token: String, dir: String) -> std::io::Result<()> {
    let  client = reqwest::blocking::Client::new();
    for elem in blob {
        let body = &client
            .get(&format!("{}/v2/library/{}/blobs/{}", REGISTRY_URL, image, elem))
            .bearer_auth(token.clone())
            .send().unwrap()
            .bytes().unwrap().to_vec();

        let decoder = libflate::gzip::Decoder::new(body.as_slice())?;
        let mut archive = tar::Archive::new(decoder);
        archive.set_preserve_permissions(true);
        archive.set_unpack_xattrs(true);
        archive.unpack(dir.clone())?;
    }

    Ok(())
}
