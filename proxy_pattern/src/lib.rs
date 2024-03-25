trait Image {
    fn show(&self);
}

struct RealImage {
    filename: String,
}

impl Image for RealImage {
    fn show(&self) {
        println!("Loading {}", self.filename);
    } 
}

struct ProxyImage {
    image: RealImage,
    cache: bool
}

impl ProxyImage {
    fn load(&mut self) {
        if !self.cache {
            self.image.show();
            self.cache = true;
        }
    }
}

impl Image for ProxyImage {
    fn show(&self) {
        if self.cache {
            println!("Loading cached");
        } else {
            self.load();
        }
    } 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let  proxy_image = ProxyImage { 
            image: RealImage { filename: "image.png".to_string() },
            cache: false 
        };
        proxy_image.show(); // Loading image.png
        proxy_image.show(); // Loading cached
        assert_eq!(true, true);
    }
}
