using System;

namespace game
{
    class Program
    {
        static void Main(string[] args)
        {
            //var instace = new GameInstance(4);

            //Console.WriteLine(instace);

            //var possibles = instace.PossibleMoves();
            //possibles.ForEach(p => Console.Write(p));
            //Console.WriteLine();

            //while (true)
            //{
            //    var input = Console.ReadKey();
            //    Console.WriteLine();

            //    int dir;
            //    if (input.Key == ConsoleKey.LeftArrow)
            //        dir = 0;
            //    else if (input.Key == ConsoleKey.UpArrow)
            //        dir = 1;
            //    else if (input.Key == ConsoleKey.RightArrow)
            //        dir = 2;
            //    else if (input.Key == ConsoleKey.DownArrow)
            //        dir = 3;
            //    else
            //        continue;

            //    possibles = instace.Play(dir);
            //    Console.WriteLine(instace);

            //    possibles.ForEach(p => Console.Write(p));
            //    Console.WriteLine();
            //}

            var bot = new Bot();

            bot.PlayOneGame();
        }
    }
}
