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

            //    var reward = instace.Play(dir);
            //    if (instace.PossibleMoves().Count == 0)
            //    {
            //        break;
            //    }
            //    Console.WriteLine(instace);

            //    possibles.ForEach(p => Console.Write(p));
            //    Console.WriteLine();
            //}

            //Console.WriteLine(instace.FinalScore());


            var bot = new Bot();
            bot.Learn(10000);

            //for (var i = 0; i < 1000; i++)
            //{
            //    bot.PlayOneGame(.8);
            //}

            //for (var i = 0; i < 1000; i++)
            //{
            //    bot.PlayOneGame(.6);
            //}

            //for (var i = 0; i < 1000; i++)
            //{
            //    bot.PlayOneGame(.4);
            //}

            //for (var i = 0; i < 1000; i++)
            //{
            //    bot.PlayOneGame(.2);
            //}

            //for (var i = 0; i < 10; i++)
            //{
            //    bot.PlayOneGame(0);
            //}

            //Console.WriteLine("INTERSECTIONS: " + bot.intersections);


            //var game = new GameInstance(3);

            //game.Transforms();
        }
    }
}
