use tonic::{transport::Channel, Request};
use address_book::address_book_service_client::AddressBookServiceClient;
use address_book::{Person, GetPersonByNameRequest};

pub mod address_book {
    tonic::include_proto!("addressbook"); // The name matches the proto package name
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a gRPC client connected to the Node.js gRPC server
    let mut client = AddressBookServiceClient::connect("http://localhost:50051").await?;

    // Add a new person to the address book
    let request = Request::new(Person {
        name: "John".to_string(),
        age: 30,
    });

    let response = client.add_person(request).await?;
    println!("Added person: {:?}", response.into_inner());

    // Get a person by name from the address book
    let request = Request::new(GetPersonByNameRequest {
        name: "John".to_string(),
    });

    let response = client.get_person_by_name(request).await?;
    println!("Got person: {:?}", response.into_inner());

    Ok(())
}
