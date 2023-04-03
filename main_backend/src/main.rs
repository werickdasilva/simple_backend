use server;

fn main() {
    let app = server::app();

    app.listen(8080)
}
