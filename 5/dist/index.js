"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const fs_1 = __importDefault(require("fs"));
fs_1.default.readFile('input.txt', (err, data) => {
    if (err) {
        console.log('error reading input file');
    }
    let input = data.toString('utf-8').split('\n');
    let seatIDs = [];
    let seats = [];
    for (let seat of input) {
        let row, col;
        [row, col] = getChairPosition(seat);
        seats.push({ row: row, col: col });
        let id = row * 8 + col;
        seatIDs.push(id);
    }
    console.log(`Max seat id is ${Math.max(...seatIDs)}`);
    let plane = [];
    for (let i = 0; i < 128; i++) {
        plane[i] = [];
        for (let j = 0; j < 8; j++) {
            plane[i][j] = ' ';
        }
    }
    for (let seat of seats) {
        plane[seat.row][seat.col] = 'x';
    }
    for (let i = 0; i < 127; i++) {
        for (let j = 0; j < 8; j++) {
            if (plane[i][j - 1] == 'x' && plane[i][j] == ' ' && plane[i][j + 1] == 'x') {
                console.log(`Your seat id is ${i * 8 + j}`);
            }
        }
    }
});
const getChairPosition = (specificator) => {
    let rowBin = specificator.substr(0, 7).replace(/B/gi, '1').replace(/F/gi, '0');
    let colBin = specificator.substr(7, 3).replace(/R/gi, '1').replace(/L/gi, '0');
    let col = parseInt(colBin, 2), row = parseInt(rowBin, 2);
    return [row, col];
};
//# sourceMappingURL=index.js.map