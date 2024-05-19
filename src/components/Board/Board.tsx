"use client";
import { invoke } from "@tauri-apps/api/tauri";
import React, { useState } from "react";
import Square from "../Square/Square";
import "./Board.css";

interface BoardProps {
  playerBoards: boolean[];
  oppBoards: boolean[];
  canPut: boolean[];
}

const Board: React.FC = () => {
  const [playerBoards, setPlayerBoards] = useState(
    Array(64)
      .fill(0)
      .map((_, i) => (i === 27 || i === 36 ? true : false))
  );
  const [oppBoards, setOppBoards] = useState(
    Array(64)
      .fill(0)
      .map((_, i) => (i === 28 || i === 35 ? true : false))
  );
  const [beforePlayerBoards, setBeforePlayerBoards] = useState(playerBoards);
  const [beforeOppBoards, setBeforeOppBoards] = useState(oppBoards);
  const [canPut, setCanPut] = useState(
    Array(64)
      .fill(0)
      .map((_, i) =>
        i === 20 || i === 29 || i === 34 || i === 43 ? true : false
      )
  );

  const handleClick = async (i: number) => {
    const gameInfo = {
      player_board: playerBoards,
      opponent_board: oppBoards,
      x: i % 8,
      y: Math.floor(i / 8),
    };

    const putBoard: boolean[][] = await invoke("put_piece", {
      gameInfo,
    });
    setPlayerBoards(putBoard[0]);
    setOppBoards(putBoard[1]);
    setBeforePlayerBoards(putBoard[0]);
    setBeforeOppBoards(putBoard[1]);

    const boardInfo = {
      player_board: putBoard[0],
      opponent_board: putBoard[1],
    };

    const oppPutBoard: boolean[][] = await invoke("opponent_put_piece", {
      boardInfo,
    });

    // 1秒まつ
    await new Promise((resolve) => setTimeout(resolve, 1000));
    console.log(oppPutBoard);
    setPlayerBoards(oppPutBoard[0]);
    setOppBoards(oppPutBoard[1]);
    setCanPut(oppPutBoard[2]);
  };

  const renderSquare = (i: number) => {
    if (playerBoards[i] === true) {
      return <Square value={"⚫️"} key={i} onClick={() => handleClick(i)} />;
    } else if (oppBoards[i] === true) {
      if (beforeOppBoards[i] === false && beforePlayerBoards[i] === false) {
        return <Square value={"⚪️"} red={true} key={i} onClick={() => handleClick(i)} />;
      } else {
        return <Square value={"⚪️"} key={i} onClick={() => handleClick(i)} />;
      }
    } else if (canPut[i]) {
      return (
        <Square
          value={""}
          canPut={true}
          key={i}
          onClick={() => handleClick(i)}
        />
      );
    } else {
      return <Square value={""} key={i} onClick={() => handleClick(i)} />;
    }
  };

  return (
    <div className="board">
      {[...Array(8)].map((_, row) => (
        <div key={row} className="board-row">
          {[...Array(8)].map((_, col) => renderSquare(row * 8 + col))}
        </div>
      ))}
    </div>
  );
};

export default Board;
