using System;
using System.Collections.Generic;
using System.Text;

namespace game
{
    public class Bot
    {

        private Dictionary<int, float[]> states = new Dictionary<int, float[]>();
        private Random random = new Random();

        public Bot()
        {

        }


        public void PlayOneGame()
        {
            var gameInstance = new GameInstance(3);
            var moves = gameInstance.PossibleMoves();

            do
            {
                var pos = random.Next(moves.Count);
                var dir = moves[pos];
                moves = gameInstance.Play(dir);
                Console.WriteLine(gameInstance);
            } while (moves.Count != 0);

        }
    }
}
