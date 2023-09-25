import __wbg_init, { add, App } from "./pkg/deft_web.js";


let cells = document.getElementById('othello-board');
const black = 1;
const white = 2;
const boardSize = 8;
const putAblePlace = 3;

let app;


// ページの読み込みが完了した後にinitialize関数を実行
window.addEventListener('DOMContentLoaded', (event) => {
    initialize();
});

async function initialize() {
    await __wbg_init();
    add(1, 3);

    // ボタンにイベントリスナーを設定
    document.getElementById('testButton').addEventListener('click', () => {
        alert(add(4, 3));
    });
    app = new App();
    console.log(app.get_board());
    drawBoard();

}

async function drawBoard() {
    let b = await app.get_board();
    let board = b.board;
    console.log(board);
    cells.innerHTML = '';
    for (let y = 0; y < boardSize; y++) {
        const tr = document.createElement('tr');
        for (let x = 0; x < boardSize; x++) {
            const td = document.createElement('td');
            if (board[y][x] === white) {
                td.classList.add('white');
            } else if (board[y][x] === black) {
                td.classList.add('black');
            } else if (board[y][x] === putAblePlace) {
                td.classList.add('put-able-place');
            }
            td.addEventListener('click', () => handleCellClick(x, y));
            tr.appendChild(td);
        }
        cells.appendChild(tr);
    }
}

function handleCellClick(x, y) {
    console.log(y, x);
    app.put(y, x);
    drawBoard();
}