use racoon::core::path::Path;
use racoon::core::request::Request;
use racoon::core::response::status::ResponseStatus;
use racoon::core::response::{HttpResponse, Response};
use racoon::core::server::Server;

use racoon::forms::fields::file_field::{FileField, UploadedFile};
use racoon::forms::fields::input_field::{InputField, InputFieldError};
use racoon::forms::fields::AbstractFields;
use racoon::forms::FormValidator;

use racoon::view;

struct UploadForm {
    name: InputField<String>,
    address: InputField<Option<String>>,
    profile_photo: FileField<UploadedFile>,
}

impl FormValidator for UploadForm {
    fn new() -> Self {
        let name: InputField<String> =
            InputField::new("name").handle_error_message(|error, default_errors| {
            match error {
                InputFieldError::MissingField(field_name ) => {
                    println!("Missing field name: {}", field_name);
                    return vec!["Custom field missing error".to_string()];
                }
                _ => {}
            }
            default_errors
        });

        let address = InputField::new("address");
        let profile_photo = FileField::new("profile");

        Self {
            name,
            address,
            profile_photo,
        }
    }

    fn form_fields(&mut self) -> racoon::forms::FormFields {
        // List here form fields
        vec![
            self.name.wrap(),
            self.address.wrap(),
            self.profile_photo.wrap(),
        ]
    }
}

async fn upload_profile(request: Request) -> Response {
    let form = UploadForm::new();
    match form.validate(&request).await {
        Ok(form) => {
            // Printing form values
            println!("Name: {}", form.name.value().await);
            println!("Address: {:?}", form.address.value().await);
            println!("File Name: {}", form.profile_photo.value().await.filename);
            return HttpResponse::ok().body("Upload success");
        }
        Err(error) => {
            return HttpResponse::bad_request().body(format!("{:?}", error.field_errors));
        }
    }
}

#[tokio::main]
async fn main() {
    let paths = vec![Path::new("/", view!(upload_profile))];
    let result = Server::bind("127.0.0.1:8080").urls(paths).run().await;
    println!("Failed to run server: {:?}", result);
}
