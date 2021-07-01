using System;
using System.Collections.Generic;
using System.Text;

namespace game
{
    public class GameInstance
    {

        private int size;

        /// [row][column]
        private int[] board;
        private Random rng;

        // 1 1 0
        // 0 2 0
        // 1 2 1

        // 1 1 0 0 2 0 1 2 1

        public GameInstance(int size)
        {
            this.size = size;
            board = new int[] { 3, 3, 3, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 };
            rng = new Random();
        }

        public List<int> Play(int dir)
        {
            var getPosition = GetPositisonFunc(dir);
            MoveBoard(getPosition);
            PlaceNewPiece();
            return PossibleMoves();
        }

        private void PlaceNewPiece()
        {
            var zeros = new List<int>(size * size);
            for (var i = 0; i < board.Length; i++)
            {
                if (board[i] == 0)
                {
                    zeros.Add(i);
                }
            }

            var zerosIndex = rng.Next(0, zeros.Count);
            board[zeros[zerosIndex]] = 1;
        }

        public List<int> PossibleMoves()
        {
            var moves = new List<int>();

            for (var i = 0; i < 4; i++)
            {
                if (HasMove(GetPositisonFunc(i)))
                    moves.Add(i);
            }

            return moves;
        }

        private bool HasMove(Func<int, int, int> getPosition)
        {
            for (var row = 0; row < size; row++)
            {
                if (HasMoveRow(getPosition, row))
                {
                    return true;
                }
            }

            return false;
        }

        private bool HasMoveRow(Func<int, int, int> getPosition, int row)
        {
            var hasZero = false;
            var prev = -1;

            for (var col = 0; col < size; col++)
            {
                var piece = board[getPosition(row, col)];

                if (piece == 0)
                {
                    hasZero = true;
                }
                else if (hasZero || prev == piece)
                {
                    return true;
                }

                prev = piece;
            }

            return false;
        }

        private void MoveBoard(Func<int, int, int> getPosition)
        {
            for (var row = 0; row < size; row++)
            {
                MoveRow(getPosition, row);
            }
        }

        private void MoveRow(Func<int, int, int> getPosition, int row)
        {
            var newCol = 0;
            var newPosition = getPosition(row, newCol);
            for (var col = 0; col < size; col++)
            {
                var position = getPosition(row, col);
                var piece = board[position];

                if (piece != 0)
                {
                    board[position] = 0;
                    if (board[newPosition] == 0)
                    {
                        board[newPosition] = piece;
                    }
                    else if (board[newPosition] == piece)
                    {
                        board[newPosition]++;
                        newCol++;
                        newPosition = getPosition(row, newCol);
                    }
                    else
                    {
                        newCol++;
                        newPosition = getPosition(row, newCol);
                        board[newPosition] = piece;
                    }
                }
            }
        }

        private Func<int, int, int> GetPositisonFunc(int dir)
        {
            switch (dir)
            {
                case 0: return Dir0Position;
                case 1: return Dir1Position;
                case 2: return Dir2Position;
                case 3: return Dir3Position;
                default: throw new Exception();
            }
        }

        private int Dir0Position(int row, int col)
        {
            return row * size + col;
        }

        private int Dir1Position(int row, int col)
        {
            return row + col * size;
        }

        private int Dir2Position(int row, int col)
        {
            return row * size + size - col - 1;
        }

        private int Dir3Position(int row, int col)
        {
            return row + size * (size - col - 1);
        }

        public override string ToString()
        {
            var builder = new StringBuilder();
            for (var i = 0; i < size; i++)
            {
                for (var j = 0; j < size; j++)
                {
                    builder.Append(board[i * size + j]);
                    builder.Append(' ');
                }
                builder.AppendLine();
            }
            return builder.ToString();
        }
    }
}
