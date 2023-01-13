// 런타임 에러__-
// function printRightTriangle(size) {
//   for (var i = 1; i <= size; i++) {
//     console.log("*".repeat(i));
//   }
// }
// printRightTriangle(5);

// 틀렸..
// const input = Number(require("fs").readFileSync("/dev/stdin").toString());

// let answer = "";
// for (const i of range(1, input+1)) {
//     answer += "*";
//     console.log(answer);
// }

// function* range(start, end) {
//     for (let i = start; i <= end; i++) {
//         yield i;
//     }
// }


// 통과1
// while 문으로 변경
const input = Number(require("fs").readFileSync("/dev/stdin").toString());

let i = 1;
let answer = "";
while (i <= input) {
    answer += "*";
    console.log(answer);
    i++;
}

// 통과2
// break 문  + for 문
const input = Number(require("fs").readFileSync("/dev/stdin").toString());

let answer = "";
for (;;) {
    answer += "*";
    console.log(answer);
    if (answer.length === input) break;
}

// 통과3
// break 문  + while 문
const input = Number(require("fs").readFileSync("/dev/stdin").toString());

let answer = "";
while (true) {
    answer += "*";
    console.log(answer);
    if (answer.length === input) break;
}
