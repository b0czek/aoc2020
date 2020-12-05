import fs from 'fs';
import { parse } from 'path';
fs.readFile('input.txt', (err, data) => {
    if (err) {
        console.log('error reading input file');
    }
    let input: string[] = data.toString('utf-8').split('\n');
    let seatIDs: number[] = [];
    let seats: Seat[] = [];
    for (let seat of input) {

        let row: number, col: number;

        [row, col] = getChairPosition(seat);
        seats.push({ row: row, col: col });

        let id = row * 8 + col;
        seatIDs.push(id);
    }

    console.log(`Max seat id is ${Math.max(...seatIDs)}`);

    // part 2
    
    let plane: Array<Array<string>> = [];
    for(let i = 0; i < 128; i++) { // fill the array
        plane[i] = [];
        for(let j = 0; j< 8; j++) {
            plane[i][j] = ' '
        }
    }

    for(let seat of seats) {
        plane[seat.row][seat.col] = 'x';
    }


    for(let i = 0; i<127; i++) {
        for(let j = 0; j<8; j++) {
            if(plane[i][j-1] == 'x' && plane[i][j] == ' ' && plane[i][j+1] == 'x') {
                console.log(`Your seat id is ${i*8+j}`);
            }
        }
    }


});

const getChairPosition = (specificator: string): [number, number] => {
    let rowBin: string = specificator.substr(0, 7).replace(/B/gi, '1').replace(/F/gi, '0');
    let colBin: string = specificator.substr(7, 3).replace(/R/gi, '1').replace(/L/gi, '0');
    let col: number = parseInt(colBin, 2), row: number = parseInt(rowBin, 2);

    return [row, col];
};

interface Seat {
    row: number,
    col: number
}