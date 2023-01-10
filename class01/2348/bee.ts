// function printRightTriangle(size: number): void {
//     for (let i = 1; i <= size; i++) {
//         console.log('*'.repeat(i));
//     }
// }
// printRightTriangle(5);

// 런타임에러1
// const readline = require("readline");

// function printRightTriangle(size: number): void {// 삼각형의 크기를 결정하는 크기 매개 변수를 사용
//   // for 문 사용하여 행 반복
//   for (let i = 1; i <= size; i++) {
//     console.log("*".repeat(i));
//   }
// }

// const rl = readline.createInterface({
//   input: process.stdin,
//   output: process.stdout,
// });

// rl.question("Enter the size of the right triangle: ", (answer) => {
//   const size = parseInt(answer);
//   printRightTriangle(size);
//   rl.close();
// });

// 런타임에러2
// import * as readline from 'readline';

// function printRightTriangle(size: number): void {
//   // Use the size parameter to determine the size of the triangle
//   // Use a for loop to repeat rows
//   for (let i = 1; i <= size; i++) {
//       console.log('*'.repeat(i));
//   }
// }

// const rl = readline.createInterface({
//   input: process.stdin,
//   output: process.stdout
// });

// rl.question('Enter the size of the right triangle: ', (answer) => {
//   const size = parseInt(answer);
//   printRightTriangle(size);
//   rl.close();
// });

// 런타임 에러3
// import * as fs from 'fs';

// function printRightTriangle(size: number): void {
//   for (let i = 1; i <= size; i++) {
//       console.log('*'.repeat(i));
//   }
// }

// fs.readFile('triangle_size.txt', 'utf8', (err, data) => {
//   if (err) {
//     console.error(err);
//   } else {
//     const size = parseInt(data);
//     printRightTriangle(size);
//   }
// });
