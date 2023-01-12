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

// 변경 이유
// split으로 이미 공백('';띄어쓰기)가 제거되었으므로 다시 확인하는 것은 의미가 없어서
// 비어있지 않은 단어 수만 계산할 수 있도록 변경 
const input = require("fs").readFileSync("/dev/stdin").toString().trim();
const words = input.split(' ');

let count = 0;
for (var i = 0; i < words.length; ++i) {
    if(words[i] !== ''){
        count++;
    }
}
console.log(count);
