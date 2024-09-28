# gRPC Node.js Server and Rust Client

This project demonstrates how to create a **gRPC server** using **Node.js** and interact with it using a **Rust client**. The server allows adding and retrieving a person's information in an address book, while the Rust client communicates with it via gRPC.

## Prerequisites

- **Node.js** (v12.x or later)
- **Rust** (installed via [rustup](https://rustup.rs/))
- **Protobuf compiler** (`protoc`)

## Project Setup

### Node.js gRPC Server

1. Initialize the project and install dependencies:

   ```bash
   npm init -y
   npm install @grpc/grpc-js @grpc/proto-loader
   ```

2. Create the `a.proto` file:

3. Implement the gRPC server in `index.ts`:

4. Compile and run the server:

   ```bash
   tsc -b
   node index.js
   ```

5. Build and run the Rust client:

   ```bash
   cargo run
   ```

## Example Code Snippets

### Node.js (gRPC Server)

```typescript
function addPerson(call, callback) {
  let person = { name: call.request.name, age: call.request.age };
  PERSONS.push(person);
  callback(null, person);
}
```

### Rust (gRPC Client)

```rust
let request = Request::new(Person { name: "Alice".to_string(), age: 28 });
let response = client.add_person(request).await?;
println!("Added person: {:?}", response.into_inner());
```

## Running the Project

1. **Run the Node.js gRPC server**:

   ```bash
   node index.js
   ```

2. **Run the Rust client**:
   ```bash
   cargo run
   ```

The Rust client will add a person and communicate with the Node.js server using gRPC.
