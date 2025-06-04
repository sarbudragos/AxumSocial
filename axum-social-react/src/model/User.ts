export class User {
    id: Number;
    username: String;
    email: String;

    constructor(id = 0, username = "", email = "") {
        this.id = id;
        this.username = username;
        this.email = email;
    }
}