using System;
using System.Collections;
using System.Collections.Generic;
using System.Linq;
using System.Text;

namespace game
{
    public class Board
    {
        /// Allows the board to be access like a array
        public byte this[int index]
        {
            get => _board[index];
            set => _board[index] = value;
        }

        private readonly int _size;
        private readonly byte[] _board;

        public Board(int size)
        {
            _size = size;
            _board = new byte[size * size];
            //board = new int[] { 0, 1, 2, 3, 4, 5, 6, 7, 8 };
        }

        public int Rotate0(int row, int col) => StartLowSizeBySize(row) + StartLowOneByOne(col);
        public int Rotate1(int row, int col) => StartLowOneByOne(row) + StartHighSizeBySize(col);
        public int Rotate2(int row, int col) => StartHighSizeBySize(row) + StartHighOneByOne(col);
        public int Rotate3(int row, int col) => StartHighOneByOne(row) + StartLowSizeBySize(col);
        public int Rotate0T(int row, int col) => StartLowSizeBySize(row) + StartHighOneByOne(col);
        public int Rotate1T(int row, int col) => StartHighOneByOne(row) + StartHighSizeBySize(col);
        public int Rotate2T(int row, int col) => StartHighSizeBySize(row) + StartLowOneByOne(col);
        public int Rotate3T(int row, int col) => StartLowOneByOne(row) + StartLowSizeBySize(col);

        int StartLowOneByOne(int v) => v;
        int StartLowSizeBySize(int v) => v * _size;
        int StartHighOneByOne(int v) => _size - v - 1;
        int StartHighSizeBySize(int v) => _size * (_size - v - 1);

        /// All the transformations
        public void Transforms()
        {
            var funcs = new Func<int, int, int>[]{
                Rotate0,
                Rotate1,
                Rotate2,
                Rotate3,
                Rotate0T,
                Rotate1T,
                Rotate2T,
                Rotate3T
            };

            foreach (var func in funcs)
            {
                for (int row = 0; row < _size; row++)
                {
                    for (int col = 0; col < _size; col++)
                    {
                        var pos = func(row, col);
                        Console.Write(_board[pos].ToString() + " ");
                    }
                    Console.WriteLine();
                }
                Console.WriteLine();
            }
        }

        public override string ToString()
        {
            var builder = new StringBuilder();
            for (var i = 0; i < _size; i++)
            {
                for (var j = 0; j < _size; j++)
                {
                    builder.Append(_board[i * _size + j]);
                    builder.Append(' ');
                }
                builder.AppendLine();
            }
            return builder.ToString();
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

            for (var row = 0; row < _size; row++)
            {
                for (var col = 0; col < _size; col++)
                {
                    var lowest = ulong.MaxValue;
                    foreach (var tranformState in tranformStates)
                    {
                        var pos = tranformState.Func(row, col);
                        var piece = _board[pos];
                        tranformState.State += piece + (ulong)Math.Pow(16, row * _size + col);
                        if (tranformState.State < lowest)
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

    class TranformState
    {
        public Func<int, int, int> Func { get; set; }
        public ulong State { get; set; }
    }
}
