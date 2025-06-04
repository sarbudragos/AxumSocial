export class Post {
    id: Number;
    content: String;
    creation_date: String;

    constructor(id = 0, content = "", creation_date = "") {
        this.id = id;
        this.content = content;
        this.creation_date = creation_date;
    }
}