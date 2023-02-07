namespace AdventOfCode2022.Days
{
    static class Day6
    {
        public static void Solve()
        {
            Console.WriteLine("Day 6, part 1: " + Solve(FullInput, 4));
            Console.WriteLine("Day 6, part 2: " + Solve(FullInput, 14));
            Console.WriteLine();
        }

        static int Solve(string input, int distinctCharsCount)
        {
            int crtIndex;
            Dictionary<char, int> chars = new();
            for (int i = 0; i < distinctCharsCount; i++)
                CreateOrIncrement(input[i]);

            for (crtIndex = distinctCharsCount; chars.Count < distinctCharsCount && crtIndex < input.Length; crtIndex++)
            {
                RemoveOrDecrement(input[crtIndex - distinctCharsCount]);
                CreateOrIncrement(input[crtIndex]);
            }
            return crtIndex;

            void CreateOrIncrement(char c)
            {
                if (chars.ContainsKey(c))
                    chars[c]++;
                else
                    chars[c] = 1;
            }
            void RemoveOrDecrement(char c)
            {
                if (chars[c] == 1)
                    chars.Remove(c);
                else
                    chars[c]--;
            }
        }

        #region Input

        private const string ShortInput = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";

        private const string FullInput = "rdzrddbgdbbqtbqbrrznznjzjjctcbtttrvrwwsvwssjfjcjnjzzqgzgzsggfddvnnrbbwgwfwgfgpgrprgrprmmqjmqmvmlmmgjmmgqmqppqgqlqmllszsmzzwfzfqzzgcccmggvfgvvzlvzlznnjfjsscrsrprcrscsqsfqfggrrhccpnpwphwhchfhtfhttzpzwzjzqzjjqjdqqnvqnqjqbqcbqccdllbcblbjljgjbgbhhgphghjhrrvbrvbrvvfggwbwnnghgwhhpbpgphpnpgpqqzvqzqtzzngzngnjnnffgbgbvvjvqvwwqnwqnnvrnnfgfttcftcfczzpbbdlbdbhbssrggtfgfbgfgfnffqsfqfcqqqwvvqpphfpfcfrrjmjhjrjbblcbbznzwwpbwwsvvwnvvzjvzzmllrpllqffplpwlpwwlpwwmnmrrlggvssjggdjjsffbtthlhsswvwjjbtbjbsswqsshppddcjjlrlttwgtwwcnwcncmcpmpnnhndnccqscqqzqpplglvvpggcvvhnhmhqqsvvhcvvjpphbhnndpdbdndjnnmccmjmhmrrlcrrfzfnndldffstffhqffhcffhbhwhzhnhrhprhhtqtztntnptnppjdpdqdzqqpzprrqbqmqhmqqmhqqjttlrrqbqnqdnnrqqtqfqjjdfjfdjjsnspptltlgttmctcgttcthcttdpppzphpnhhvthtccbvccpplpwwfgwfffhfbbrsbbgppnhpnhhwnnnrprcpcncffrqqpjprpddjnddnccqtcthhfqfnnnlrrzhhmvvczzlbzbrzbbjpjpbbjttmwtwbtbqqtssgdsdsmsmbsmsfftltmtwwsswbwdbdwbbblgbbqpbqbmbgmbgmbbvcvbbmrrldlpdpqqnvvrzzcddjsdjjfvjvvndnvvrsvrsvrsrmsswlswlssqllfmlffmwmttjgttzgtggghrhprppbgbdgdtgddvmdvvbmmchhjppczzglzglzglzgggsmsrrsttgddmvvhddvdhvvcfcvvglvlmvmdvmdvdcdvvgfftccljllmssrvvqhvvnlnplnlglrlvrvfvrrgjjzqjjzcjcvcsvvbppgrrbdrdsrddqsspwssdlltlcttgngzglgfgzfzhfzhfhnffmsspvsscjjrbbpgbbfgbffhggzpznzmnmcmzmdzmzjzddcsscgssdnddjbdjdtdzdwwshwhfhtthjhsjjfjvvdnnmzmtmmlrlblzzmmspsnpnbnvvczvzsszrrsgsbsrrlprrzfrrjdrdbdhdvvwrvvrfvrrvwrrqlqwwqjjpwwffdvffwfbwwdrwdrddhhhjghjhvhccgbcbvblllbwlwtlldbdzbbdpbbqmmglmllvwwzmzhhfnfcnfcfddvrvhvbvzvbvhvthtrtppsccggdgpghgfhhhnwnmmwfwtffpjfppmdpdhppvwvfvvhdhzhtzhtzzplpspmpnpzpgzgqgvvqcqcqmqffzszwzfwfzzcmchhmgmppgwwvffnnvhhttvtjvjgjljmmfcmmzmvvjfjggppqmpqqswsnwwdvwdvwdwjwrrrtprpqrrsddsshgssbbpbgpbbprprzppsnsbszszccddfcftcfczffchfcfjjffjwwsbbvfvpfprpbczqdnvrqrlhsrzvlvgzbqpwgpmgftzfvtlnlqrtmpjfstmsrbfjldtmhvvqwznrcflbvmsnbzcrjvbzgvvmwlzcwggwvhnpscnwhjldzjvmtfdgvptfhjtdtwfjntzqtswqgwvnfgqhqpdvdchfqjzfhgjmdstmchppcvgpdfrqbrhrvdvzbtnnrpqsnmnljjzgzqfnzmlvbzmwzfbfszvfsqdnqfstpjglwtcdjpmdfqzfcsngbcvvjvdzlnzndcggcdtdjpwgvfzlfdqpgbgfjjfvgtwwrgtrfcpfvmzvrwrftpfdprzmcmrpnjpfrdvbmlrfzrjcdzhvwbpgmbpdnqggjdpqttgnqbtjfmrglqbvcfjwghrqtcgjddhwpntjgtghmfgvjdhbzzgmfnrrrgvdmsfbhndfcwlbbtsdcbpgfpbvnwmpwdpjlvcbcgnwjzftsfqhvwdshzltrmqpcsngfzrvfwhffrcjzlfqjqdbcntdwhfrfnrzwftqhlfjpjqpngjqjcnfnrpmmbprdtzgsvsdjpzsnzzmzdfjplfhzqrnrwggcvrrmqzwlvslwvtvhbgwmjmnzftrbfhdrzszcvmdhgfvwgwgsgqtpwgvpvfrszczmsmstwhnhtnftmmmjblpchrnrdwlnhrvqtbwqwdchfjbcldmjmjzwlcngfgfvmblgmnwtwvjmzswzmpvdjtcfgcpvvqhrfnwcczrrrhwwvfrngwmlstqcvvrqrshwdvrgtgbffvlwtvwhvlcwwqgspnfmndbsbpvdbjwdlfntwrrmtgsdtwbfmjjcjfvsvltwdjmvswwpchpdtsjmbbgcgtddcbprwznsldmwflrgcrgfflzwllrzcrgdgqgsvrqmspqzsrzvppztrhlpsnqdlhmghdcrppvbljsnrjgcwhpvmlgcnswrjjbjltwnctqbqsffbcfpclhcbrnsjlmrstpngbvcfcbjstgvzwfgcsqbwgqqblsnmfddprrqmpqgfrjbhfptrdvltcrmtrcqgcdfpjhjptzngzqdghqhpsfwlmrwgfhldzpfrbtzzgsblcfmwztjmtjzgwrpttvwfhbntvsgnfvbpfpnscdspmcvncsqltzqnwczvdbtphwrdtjcszmhbpcfnvbrbgfpnrrhvhzlrglthpldrlfscpvpvttvjtfdrqpjvnwvmscdvclbnzfvppslzgmglrvvdvpsbsfhflnjbdnpqzjzlrfgwgjvwvpthjptftlzppmnmrrpfvdhgwfzfdnnbhqpwrzvvdgtcrflwfjgbmvbgsnqzqtmsvrdlfmlbqqdnfftljbtphnpqqrgbrlzrbbhlgvjpcdbmtvzlpqhbvjpmhpdmtrmjwvzrbfrdrdfmwsfvllljdwmnqlgcnzvpphwmlmstcsljvmljcjprtgmzfssgbtjlttssfzbcqbgnjnvrwzchtwtwdtfwngdflwzjrhzdlrrqfnsztvdbzqfwfzppqrghrhzsqtwqstsfspddpjrpffhgqvspzwmlzwhtzqqldqbwlsrqhlmvhhmzjpdsrgdcvqnfpldnmblgvvssrcdnjqvcgwtwmhqdcwtsqqhbntvjnlbljlqjqglggvpqncpdzztlvhhlghtrncfcdhjtzwjqdhlntjfrzccbnmglmnzwvplclvcmnsppqjhbggbzncqlcfnzbbzdjrvcdthcqwjzjvdbjddcbjchwfgbjhwqpzbpgsdtlwlphtvhwddjdbwbpsnqhnffqptcrljcqzzhszdfpdfsgflhwwsgbfcnwrbdqflnrcwddwwmfztlbswlzhtzzcllnvbtqgjsdzmhcpnnqpdpqgdntlfwgvddgqvhqhrbvstsmzrmgwslpdjlsbgthfhgnlftbqdnzsvcrrmllcjvdlqrzbvrbrjcbpttsvwcrnlvvbnjvfgldzmtflvmzqdgbnjcgctllrldgzltwlswmfbbwjmcqpldhhdsmqpbvnjprdqnvrbjhrjzqwqfrfqwngwtwjjmzdbqmpmvqrprjhnhnrlmlgpfwwzjhlgmbzdlpshcqpnlgrqvprbspmdznzzsvhdlzwmttpdnlrlqjllqnshjllvvsrblscjcmbcqlsgpcjlmmpgwrvjnjzvzfgvgghwqfjswjbjghmzcgdpsjwhbnzmbhtzgnchpbrmnfdbfscgzldpqmvprjpvcwtwdfjblfshffwqctdphhnhngsjlrqtqprpjhwqcbmhctqbpdtpzvbbfncfrcvmbfvqmbmqjvgtdvspfqfbqnmjwhzbcpcfgbhtllbsssqntfbmsmlwhjchgcsrvsfznbmspwwszqfnwzzljfnvcwnwmgfzqfvmwwwdjd";

        #endregion
    }
}
