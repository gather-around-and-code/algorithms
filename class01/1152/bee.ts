const fs = require("fs");
const input = fs.readFileSync("/dev/stdin").toString().trim();
const words = input.split(" ");

let count: number = 0;
for (let i = 0; i < words.length; ++i) {
    if (words[i] !== "") {
        count++;
    }
}
console.log(count);
