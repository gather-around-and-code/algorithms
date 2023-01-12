// const inputString = 'The first character is a blank'
// const inputString = ' The first character is a blank'
// const inputString = 'The last character is a blank '

// 런타임에러
// const input = require("fs").readFileSync("/dev/stdin").toString().trim();
// const words = input.split(' ');
// for (var i = 0; i < words.length; ++i) {
//     if(words[i] === ' '){
//         return console.log(words.length);
//     }else if(words[i] !== ''){
//         return console.log(words.length -1);
//     }
// }

const input = require("fs").readFileSync("/dev/stdin").toString().trim();
const words = input.split(' ');

let count = 0;
for (var i = 0; i < words.length; ++i) {
    if(words[i] !== ''){
        count++;
    }
}
console.log(count);
