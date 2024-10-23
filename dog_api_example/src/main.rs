struct DogImage {
}

#[derive(Debug)]
enum ApiResult {
    Success(DogImage),
    ApiError(String),
    NetworkError(String),
}

fn main() {
    let url = "https://dog.ceo/api/breeds/image/random";

    let req = ureq::get(url).call();
    let content = req.intro_json::<DogImage>();

    println!("{:?}", content);
}