using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;

namespace game
{
    public class GameInstance
    {
        private int size;

        private Board board;
        private Random rng;

        // 1 1 0
        // 0 2 0
        // 1 2 1

        // 1 1 0 0 2 0 1 2 1

        public GameInstance(int size)
        {
            this.size = size;
            board = new Board<byte>(size);
            rng = new Random();
            PlaceNewPiece();
        }

        public int Play(int dir)
        {
            var getPosition = GetPositisonFunc(dir);
            var score = MoveBoard(getPosition);
            PlaceNewPiece();
            return score;
        }

        private void PlaceNewPiece()
        {
            var zeros = new List<int>(size * size);
            for (var i = 0; i < board.Length; i++)
            {
                board.get
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

        private int MoveBoard(Func<int, int, int> getPosition)
        {
            var score = 0;

            for (var row = 0; row < size; row++)
            {
                score += MoveRow(getPosition, row);
            }

            return score;
        }

        private int MoveRow(Func<int, int, int> getPosition, int row)
        {
            var score = 0;
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
                        score++;
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

            return score;
        }

        private Func<int, int, int> GetPositisonFunc(int dir)
        {
            switch (dir)
            {
                case 0: return Rotate0;
                case 1: return Rotate3;
                case 2: return Rotate2;
                case 3: return Rotate1;
                default: throw new Exception();
            }
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

        public int FinalScore()
        {
            var sum = 0;

            for (var i = 0; i < board.Length; i++)
            {
                sum += board[i];
            }

            return sum;
        }

        public ulong GetState()
        {
            var tranformStates = new List<TranformState>
            {
                new TranformState{ Func = Rotate0, State = 0 },
                new TranformState{ Func = Rotate1, State = 0 },
                new TranformState{ Func = Rotate2, State = 0 },
                new TranformState{ Func = Rotate3, State = 0 },
                new TranformState{ Func = Rotate0T, State = 0 },
                new TranformState{ Func = Rotate1T, State = 0 },
                new TranformState{ Func = Rotate2T, State = 0 },
                new TranformState{ Func = Rotate3T, State = 0 },
            };

            for (var row = 0; row < size; row++)
            {
                for (var col = 0; col < size; col++)
                {
                    var lowest = ulong.MaxValue;
                    foreach (var tranformState in tranformStates)
                    {
                        var pos = tranformState.Func(row, col);
                        var piece = board[pos];

                        tranformState.State += (ulong)piece * (ulong)Math.Pow(16, row*col);
                        if(tranformState.State < lowest)
                        {
                            lowest = tranformState.State;
                        }
                    }

                    tranformStates = tranformStates.Where(tranformState => tranformState.State == lowest).ToList();
                }
            }

            return tranformStates[0].State;
        }
    }

}
