﻿namespace AdventOfCode2022.Days
{
    static class Day3
    {
        public static void Solve()
        {
            Console.WriteLine("Day 3, part 1: " + Part1(FullInput));
            Console.WriteLine("Day 3, part 2: " + Part2(FullInput));
            Console.WriteLine();
        }

        private static int Part1(string input)
        {
            return input.Split(Environment.NewLine)
                        .Select(x => x.Trim())
                        .Where(x => !string.IsNullOrEmpty(x))
                        .Select(str => (str[..(str.Length / 2)], str[(str.Length / 2)..]))
                        .Select(x => x.Item1.Intersect(x.Item2).First())
                        .Select(Priority)
                        .Sum();
        }

        private static int Part2(string input)
        {
            return input.Split(Environment.NewLine)
                        .Select(x => x.Trim())
                        .Where(x => !string.IsNullOrEmpty(x))
                        .Chunk(3)
                        .Select(GetBadge)
                        .Select(Priority)
                        .Sum();

            char GetBadge(IEnumerable<string> contents)
            {
                var contentsAsArray = contents.ToArray();
                return contentsAsArray[0].Intersect(contentsAsArray[1].Intersect(contentsAsArray[2])).First();
            }
        }

        private static int Priority(char c)
            => (c - 'a') < 0 ? c - 'A' + 27 : c - 'a' + 1; // not very pretty, but good enough

        #region Input

        private const string ShortInput = @"
            vJrwpWtwJgWrhcsFMMfFFhFp
            jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
            PmmdzqPrVvPwwTWBwg
            wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
            ttgJtRGJQctTZtZT
            CrZsJsPPZsGzwwsLwLmpwMDw";

        private const string FullInput = @"
            gfWpjRRQffQGCHHJsGqjsj
            SclzJZZvmmnPbJtVSqqNBqVCBdSCsd
            tlbvZJDZtmtPcJmlPnhMFQWWpMRFTfLDRRTWRp
            HjMPgSWjVrjgbHRRSSMRgjRdpdbGdlcdCvQfcCdlwQJfdf
            LNDnhtNtLNFFZDtFnhzvdldDflvvDCdlJfldpJ
            ZFLFZZmFtFtTNTSPRrVPWWMpRP
            qLBSBLRwmgzqCbzCffDlrfCV
            TFFFHNWFMFFMpHpGHMTHGNhrldWZCsdZsslZlZfrflDVss
            PTMcPGntTThHhTGctnMvSwjjvmmqLBmnjqqgCR
            nClJtMwwntqVVPJcgZqq
            mjpsDcrcSSFFPZqFBWWgVP
            vQcjsvhrvvrmhbmNHMNnlHbNMtCtNM
            bgvvhnTQtjrrrhsDDf
            pLSMltLzLLSjFrSSjrSJHD
            zNWRLBdZPllPQtCvttgCqb
            DRlDrrFTNDNlgzsGTBfcnqhhcnJfcrCSqc
            MMmmdWtdLmvtldHjMmQfPBqSJWnfCCCqcWSSPJ
            vjHMjLmjpLtHptQLmHvwTRgNVVpTzZFZgZRlsVTN
            rzpMpDCGFCFFjRFsRPFRNFPv
            fWclbHCHtSmfvjnmfsvZ
            wTcTlSwwtQtWclBQBLGMLMCLVzVLwJGqLd
            MQSjLNjPPLLSBPjfQhSPHjDVCjDtVVpDHwbwVpbD
            RcmWzsRrzZrmTszWRqWlmRJscbtHwCbndCtcDVddDpdnVnbt
            JTsrGGTqmwTlWmTzJzWmhhPLLGgPFgBffSSPhFFM
            qMMRNZMDDNWLPqfzCgDcGncVDCgG
            wwBFhwhhBgmcVzhghG
            tbJbjjtJvwtdtwjpFtlbvtdTLNSMqNqMMgqNHPlZRTNggL
            qmjMHsZmZSbjbZMjSLFFFFwgsgvFswpwww
            hRJBhmnhhvFFwhcv
            llfWDWzrzBNTRfNBrWzzTmZbGTMjPqMmZPjVbSZGSP
            CRRPLwwcclcGVppQ
            SHFjDjjHDTfSDNTTHfSHjQVGrpmllQQWltVVVZGp
            HFlqzDTfqlzwbgPJLwCP
            WRCNLphpLppSCWVHNfLRzVnQMnBnMddPMQDFQgrhPQFM
            jTjJqvqjvPVJFJFBJF
            qTsZbvGqqZlstsmZVljtwqwSHHNWczHSSRcWNSRHzzNfbW
            glgzDzHjSrVHcVgbrjmNsscNGmNWssGNNtst
            hHPQLHJpwdLpdHfQQtnZmNMwnZGZWwsFZM
            QpdhPJRTJfPphJfhCBlVqVvgvVDBbvVqDbHD
            VtHzjZpjVtHrprgGmjHsGHNdSJFQRcLJqCdQcSqJNpcq
            bBWfTPwhbfDlMnhffRwQJQNdqJcLFQLSdR
            bhBhvfMWTnlDnTBfPSmvmjsjmmGtzHtsHm
            pcRPRPWrSDcJGZSStmwZZS
            VnLfCfTlfVzfnMMBCqVNZJdtjNtJjhJdGNNbwT
            BLvqCCMVsnRQsPQgDcZH
            cQbqqQhDGhlQfQlhQrqGsTNgLgCpRgLTPPPLNbpg
            wtHVddVFwSHznZwwznCpRBdjppNBNTTdCjRR
            ZtWFwWtSmvVnwZDrCMGfQlDDJQmD
            PzPZGCZzrZrlhdjdCqfCsqQdRD
            cbvZLVVFvbbNSNFHSDnsDQdnfqNQDRngsR
            FJHSLSFSScJJbWHFmFVFSZmrrzBmhtBwmzBMPMPzPh
            nlpFcLBgcVcLbssGVBGGrlpGPhJJJJJqPBZPDNMQMJJhJQZZ
            SSTjHzfHwtZSPVQVQMRQ
            TzVHwWfTtzwdVzsbFnGgsbdcGrLc
            FppVBRVZDdLmrDGmmfrQ
            NtNMPNshJCzznLGJSrqRrRrr
            tRssthhPlCWhPzsWtzhzCbVVjwTpVwdZZTpwjbdBbwBc
            TTWblHWScvPCCHTWFzSrqqsNNSmdmqrrpz
            RLRwjjnjZNprzmmZcq
            QQgtQnccQDGjgLDRRcLthQhFBvCbMtMHTWlBFllBbFCMTW
            WnBVNvDnVsNvZWdrWDLVDMbsHpTjpHCSSClsbSCCMH
            GPFtmztzgPhRFtJTdbTwjppSCjpgSl
            hJcfPtQhdtWNVZqNnqNQ
            GLcqZPPsnqQcFsmBBrqRvrddNqrC
            MtHthJwLllwvjRvvtrvBRS
            VHMfDLbpfznszZQG
            WBSdPlQPRfBtGQPfBGPBJgzgjwsJzsszJwCrdwCT
            ZpppVpMVpnVHMVVbZRJrCgwRzTJrwNJw
            MvhmnpLqLmhVmBlftRQBFSlR
            hhQlSJqhtCSnqZJnqShSlNDwRzpvdwRlMBMMdcjRjMpMRc
            frrGmLmWbfFrsmFHmBzBvBcwdJbvpjzbMM
            mmgFrVGLWJLFGsgfhSVtVPqntqnnSStN
            SFJTJTSqswwFQbwf
            cDtcWPclrtPwVsfssQmN
            HDtwWCgWdggdzSGJMSzGMq
            JpqJtWRJMhCMJpMQCWtFrjgHdgdlgllwNjlQjldH
            fBzPZcZvnBmDnZvZBZDmPvglVVVdgHHSwrNRgVgwNPRH
            GbZnZccfvcsZmccsmnnZTRbCCMWFTWJqFCCMJFRT
            vrrFqrFTBTmLmNrLMqMTHddJbHpWnhdWdWbHhJGM
            wBzfwzcQSzWSSshpdWGp
            gwjPPPDQtzQlzQDPqTgLBRmRqZBvqFNR
            bWVptFFsbPcZsGLhsZGmLB
            qnWrnrHdMCDCNqfWmvRRZSSRLdRGZGRG
            nNqqNDfMrMWHDQNHzWfHNDnwzblpzFlbwtFbVVlwVcPJpP
            BHJhlHdJQggvddglJBBhglhQzZHPZpFFPDMzFDDRDFZZDFZD
            rSTfqnCffMfCVfCLNqbzbjWNDbbWDPFpPFbP
            nfnnrSfCTVSwrqSLCGfTGlgQhlvsGMJQJBhhssJhGc
            tBjjDjjqfDjLfJlrLgglvmrlmrcc
            TwNNTVhwwpgvGSNNSssS
            TbwhnvvChhbVRTPPRJBJQQfJttMQQJCQfW
            mWSvSQVgmWQsQvspQJlrlLnJLLpCClhhlp
            bFHRjZdNjjBZzFzhtnCllCcJLrCBll
            HFFNHbdZZLZjfPFjHVQmWDDVsvsmTqVqDf
            JJPllQQClqgBCgdHwHbpjVTwHd
            tmGZtjGjHZpVbfMT
            ShGjNGWmDSNcNRtGmshDRzzCvzQJJRBLrvlrBPJv
            cTpqsTWqVVpsNLfvCDFlMFDVFL
            JnndJPddQgzHlvMJFDhLCG
            BjtntgdRnQgzjdBRQBlpNWrTTlNTSwNpWS
            qHmqLVLjmVqsDBLtmjmbtPwCTwwPzGWRgGwGwMwW
            ZhcCNCSprRTWTwSnWW
            hflhZvvQhppZfcNpvrhpQHjVjLmbVmmVHVCFDvqVFb
            nnNrwDnZrspwDNnZsNSDsNbCmpjvMTPQjLMmPmmQPGBTQP
            FdVtRdRfctBQPmTtTLQB
            qhzWVWJqVHwbhlLSsS
            htWmhDhFztnztDhtBmBtghPRSrpfjVwPdfPwpwnRSVrr
            cbCHvgJGcTqbqcbqqqcqsMsRVrSCwffdRPPpVpwCRSwfjj
            GlgGQqTqbgQzttmBNNFz
            NWQNQgdTgjQNddTZfrCQWRDnnnbqnLqnRcjJlqqvDj
            FtSSmSmJhpllcclDvpln
            JBVVSsSFBVBttShFGSPQfCGNdrMfZZTQTZNNdC
            HgHthMhphcbfbMMfHhsGGDCRRVlcVSScsCRz
            nWvPFqLqPNdjnNLnjdJnPdWjGlssDPSsllVCRzlTCTGlSDzS
            RvddJRJQHwQwpZZb
            gdZwgpjZZQtHTdrWrwdpWRnlhNBRlLbFthNhflhBnL
            CVzDCPGMVqVmGsGGbJCmCDvMcRcqnBFFFnRBBNRBBNqhnFfF
            DsmSGsGPzvMGJvdbgTSTbjbSSdgH
            jBGmbNBQGdBNNDJNQRLLVDsHtDRzHHZZcH
            wCWPFWPCrPhPrplvprhwpCHHtszttqZslRVHLtzVlJZL
            vprMMvMnJCwnnPShNGSTfGSfNmmgdNff
            bPtLbvVWWztbLSVVnbszpzQsrcDDBdpRcDrs
            llZmgCZqgCFgmdRdJcscBdJsmQ
            FZlgfqCFfgZHlqCMCglwCFGWntLLSMRSPGPVttWRtVGL
            vtnDsDtrnrSvrMVmbrrJgPCmBm
            FpQHzFclLVzWHhwHLQLlHLzPmMBQCJTdTmCTmBTJTTmgQg
            pllcVWqlffZqZtZD
            TSSZWpsQmZWcTZSvsTTTppNPzrBPrNBrzQNVFrBBNPqP
            CgjmCbtGgftMmLtLmffzBzJJJNVVMNzNBqJrFN
            gjgjLgtLwgbGjHdhhGdvmlnllnpWnplZvcvwTl
            htLrRFRtbbhlGSLRtbJBJsjBmgMMgJgtmBzz
            pZQWddQQfpZZffcDQZwddQwDMqDDsPgGJJzzjqzgJMBJgmms
            QdcQTdwpGNwfrCRlRVlNLSbb
            wrdvpVBVpMGPPjWjGZJJZT
            tChCSlNfCCHtvHHWPHPZ
            RbRRNvmcqcblfMwwdVBQQqqdpL
            qcctqRcqmcHWzHBdDMZhfwthBnwt
            JFsSNMSgNSNJJMGJBBdjhFDfhwhBrwnZ
            TbgbsSgJMTJllblLCSPlsTCVQmRVVWpQzzqpqzVzHLQzcc
            CVcWbjjSSCSSnpjWpCpprhHZlHtHGzHrZrHGclrl
            gqZqdddLgmgNqvTGGHvvmrrGHT
            FFDgZfZNLMgNfdDqDRnsnjBpbSbnMBBWpQpB
            qwpQFwRnqFFfSBSfFt
            LJJLGLWWtZlbgWHgGshhSdSVzmhHmfVzzC
            lrbrbrNNJgDMLLbblGctvvvDqPcqctTTTcqP
            vnblvbfHvlcHMlHlZbSPLTPLwCMBRRPRRFFR
            tszzBqtzDsWVPRSmzLVmVL
            tsNsDDNgGsqBrgBpgdHQbfhflcHdpZvdbh
            cCpLtpGGLsgsppcpmGGHMtjfHRVhvvVVFRfhjV
            NWnnnNNndQnQZdCdzzRVMHzvhhHWWWjj
            CPJJrnSZpGDJLGTL
            cnJzpcnmnQVFbzTlvTHBlb
            tWCDPjfsDGfZhddhjjdTvFTgFgvbnFHvdHqT
            hjfCjwDDGjPthsfhsnGNrJcQcRmJMLVJrJNMLw
            CPPRrSlRccPcwTHwfdwTHdfl
            mLQLLjhQhhQLZvpzssHDhdTswzzTJD
            gmjbBvQLWmgbQZBCSRnnnSMVCBHnBS
            sWrBJbsVqschzhQzHh
            gtFmztnSlSfdlmnZSdSwcwGRTjcTcwwTcHccRg
            FzFDzMZCdDZtCSrJVBMqWVrqNBqN
            TvWlhhfhZJVgtSSl
            ddBdGGdFmmBbdzqqPDDGGmdDZSgttHtZppSgzZHSgMhtMgtz
            PGqdrbbbdPnrcjjhTRWLLc
            trrmJWcrVwVbcPScdcBdGPHH
            JTQnfjlJTpQFfMLlNJHHGDPdGsSdDjHGDPPH
            ffFfnCTTCfTlplTMvNVzqWvwVzrrhwmWhJbW
            hVtDtgcghzJpmmhlwp
            srsnrqqsPqsBPvnqRBRMPbnwlplpmCStJwmzJPtJzJfwSw
            bbrqjBbvGsjGGBWqMVFFVDNVNjZjgtgFgZ
            mnmhBDHhwWCHsTgRsH
            dcSlFvccMFMMFFggNsTzzvvzWnVW
            llQdllZScFplJPpdcZSqBqjhmtnrwrDGnQGhrq
            ZffVNgfTdmPVltsnnGwgQDnB
            rMCFLMHpzCMFzHpzbrcHFLzBwsDsDDnlDBJrDDBBSJSnBn
            MLMjMzqpCzvwqTmwZdvq
            DDNlWPRqgPRPsRFjJQZbchJZbgQJ
            zzrLLznpLbHnjcBHvVvHvJcZ
            ndmrTzbMMTfzrTfnTLrzdpmsPPPqlqGDNNsPCRDRqRsD
            zzdqTNfTfdfhgQhgqMFSjRDtDRWHqtWlwtqDRS
            ssBCrcmpVGZvVRDdSDRwtmWdDb
            rvGPCZLCVCPVBZFdnfThgNgLJNhf
            bslcrssQwDPbQrrcsbsnQrjMLthPMMRhLRhLRgzmgPhRgM
            DffvDfHGfNFdpfTdMtghLBThzVmBhBtM
            SNvJNJdflDDbcDWJ
            HFlHNpWsTlGWbFsGFTGHFLLNzPPhLVPMzVzMNPhhzP
            jSvZtmrqqpcrCpPVzw
            dddQvqDgDmjdSQQdqZjStpffWGgBRWTGfGsRlWBlHF
            THnTbNrdBnLTHHnTnBrWRTndsccZsLZcDqmLDPcDlQDsmmsZ
            ptwzzhpvGSVdqQlmszqmqPqc
            wGVjSddCBggCHFWN
            LFFbdbhhhvwvfTNdRhhRRvMbHDGjcfcGfDjtDHHcHqGjDqqj
            WlQnVpWSSWWsPsgDqDzHDLHjJcttGP
            rrWsZrgVnWrWSlmSlmSBFFbvTThhBFvvZLBhRw
            BgBdcjThvjFcTggrqvVfzlnnPlrqLt
            JpwJGPsQwpwSssHpPLlzlnNlzLLNNLVtsN
            JPMmWGmWPmHbHpJbWGJmDmwbBTRZMBBdZCRTRjFjhCZCCBTT
            BjbcLFRfBRhnbGjCVVvPllpcPtcDmdlPpvPP
            WrMQqCNgsqWWsTNCMZMWWsWPvJDJDddvlpDtZDpDDDDwvP
            qNMzzSzSQsGLbFCSCnVR
            tTRpHJQpQBZcddhhMhvhJN
            zswljflgMFbwPqmNmSdvShLNfLhm
            qFbsMCVgsqMwRWHCWDDBDWpt
            VSTCCWsJvGpHHCNC
            GrqzZrrZjDljcDDlfjMqgRPfPvQPpBHNvHvBpvNQ
            rljncDcznjMqhlhZDnltrzhTsGWtbVLFTTWGsbdWJdFTmL
            mJPDSJJPZPJNrprSNrDmpZGrhFFhBqjGbGGVbFjhhfqBjBRV
            cgnTQHdMQdTHdhqfggBhVqVfVS
            nQdLLddssSJrmsNvZrPz
            jfjffQzZQQMzZZfZZQFgjDWBCRlCBdTTBGGGRpBCgdhdBG
            LrstWtNsbHLsprRBdlGpCwlh
            HLnntbnscqLvvPNNfMWSSmDMDPjzjDzS
            vhcGwWVvglltcfBn
            BBSLrzSJLzJNJrLfPfPRsmDRmflD
            jMjFZJNMqzrzZzFNFjNQqJzbCpBBvWdpvTCWhpVwdvHVCGbG
            HlrnFmRmtRBQPVBTQHHQ
            psSLJsLpTTdPdLTv
            fCGgTgfSSCtRtFFzql
            pfTpStppcDlWfbpDdzQRsQGJhfffQgJHzN
            ZFZFZmBFwVwBVmLmLsRLRhHNzRLRNNzJ
            FnnjwVPmnqqqjBjrTdblldCTpcPJtbTD
            bdZHdWlrjslMMwGG
            rDDTRBTqSqmJLBJRBTSJpmMsMMjhwvfMhjjfVGsLshhC
            BqQFRPFRQBJgzrcZNHFdZt
            wrDdLlDdPWZPTTrwlZpSsPsHVHsSCHnbzMHM
            JtNFttNCjFvpppnMpJgSVS
            NFFqFcCQCvfrZmGdZdmqrW
            GMNNfJnNddJFJWsv
            HSDwCmmghLmwmmHDpsvdFpMWpppptSbp
            zCzBCgzhwmhzLrPnVrMqZBNfGf
            DrHGtbltbCjjjffPrgsmzmcqsgDczdsmgJ
            VZLwQLZLLVwLBQZnLVphhLQQqsTNmzJdcNTzzmJNqlNBsszz
            wZLhVMplpQVRRlpVGPfjCjMGCrbHGWWb
            BHpFrHHbBNTWWTWNhCPwPLNPjCdjLV
            zJRRzJvZlcZsSMJdzSDjDtfDCtDtjDjjjj
            dcJcszQJJGRJzRllMpGHpFTWmrTmBTbWWB
            qnWWqhDhnjmjCMBlNRrfVfRNCB
            vvBLBtGHJTHBddrNVJrVSVdr
            BZLTHbgvHvTFBgTFFvhmWmmZDPmmZDsnqncs
            WBvmjDbSzTMmHHdpNHNF
            ttlflZRfGtfWVRltGtflCdHnJrNJHNHnJddNMNCnpF
            VVwssWQQfRGZcszBQzDbjSBvSBDP
            lSlQqQVqWWVWfqQWVJSTscdmPPwwTTmjjfpjPp
            FCbzHbvHvtgrtFCvbvbbwdTwmsrwnTTpmdswmwcc
            DtZbHdghztlLMQlWWhVQ
            pqzzFSmdFqbQvlpdDGGrGBWPPBVNQnVttZ
            cgcjwfBMhHCjjLMCrtcnPcsnsPGVnrVs
            JgCChjjjBHhRRLLjjhplzvzpSFJvzzlDbSqm
            mZzVQZMhmrffwfQhWhzmrmpBtRcdbnbcdcMpBbDbncdD
            jsLTSlTWRBSDpnDn
            GLTsGWGFsfmJGZVJZm
            BGWshBGnsFWSLWBLlSSLWRJHnrVPrPcNHCNHctnPPJ
            QmvQCqqMTZqvgmvTjpZCMgMtrVctPptHtrNVrptbJJbrRP
            CzjCZfCwDzShDWdF
            HmQlQHmJnpmptmzt
            MTqMjMPvTvVvhpdztZnSwzwZqS
            CcbLLPTMtCCsjHNHQFLRRFlRNN
            GDFwLLLLSrbdPlFBMFsslFHmZH
            TnJCgthHpVTfZMQZQmzWnZ
            hjvtjtghtqJvVjhTgNhJTvdvdDDRbbccrwPdcGwrHS
            MQQMBPzMGQBPBbDQPMhpnRwsGnRhNrFFpRnF
            vmgHcmCTTlvvvZvTmqcTfmCRdddFnwdRdnVwFpVfpRnwNw
            gvmqJTcHclCQJNzjMLWbLj
            DbqqDDbQFqfNtZSLSq
            RrdjPdmrpWBdmWRdccfLtNttSDMZBfftLMLf
            dCcgmgRrWcgcppjCVVVVFHFnDnbJnb
            fZMFfrtVdZSDVwTgjRMLhwTCLj
            cNzPBNpclllzHbmTNRhqCRTgjC
            nhhWJzhGPlQcGvsvfJtSfZfrtt
            PSzrBWQBBGzBlnSnWtDrqHfNfwVwHcLNjHjwcDNmFH
            hbRhtRCRpRvsRgVVVcNHNNNCwLwc
            ZtRTRvttWWzBPlGZ
            tcLnctNsJrWWNDTN
            pwPPSjHSHHfzvmSvvvFVVGqGVqGmFqrDWgDr
            pPSvfPQMzCQCSbhllLnQDhbtQZ
            DmLffDhpVhjjVwvbwNVFbbNSNH
            JRPBgMPRHBrMHMHqrBMqWJBSQQNbCvndNrdvCNCFwFrQnv
            WcqJcPGMGtWRRBtgZjjspGHTLHGHTppm
            ptJtWJpqRwDZZDVWpbDWqlvvflfMjlfCMjdCCdtslv
            rLwTBGBzBBQTzmwCCjvdvlLllddsMl
            NBwTmrGNgrTrcgPpWgWPDSVVPW
            CdglMnrlSSqDPpcsZb
            ccwmVJtvVvVtNhBpBFPDVpqbbD
            TRGQjJjGTmtrTCgHWLfrcn
            JNNhLwWwWQHNPDmmjHpc
            zMqZCvVCSMVqMSTVvZVGsBnlslpmsmzlPmsHPsPB
            qTVqrgdCCbhfHJQFtg
            wNwCBBCZsfQWfmLCGSmmFRGSSF
            zjnPHPVqMhhZLTcbpbSncp
            lVlhlgzlPZlwtgBddJdfvf
            JWRWRRLWJLnjtjnLzGzznflBvfPvPMqMDqdbzblCzC
            TTScTVbHmTsVFrmcsgcHFlPMMvlvrDPdlrDDqdldvl
            bVpcpchgsFZHbhSmSTsHFFjwtZjnjLttntNjLjNLWtjw
            rffjPJzWzrgPpGWHVNqTtmqFTVRH
            cswhvlLBvSLsCtbFccmqVFNTbb
            wwZSCZSnCLsSDGgDmpGnfmmr
            rTfJTNtjfNljlrWSlzRtNlTqsddwGnsnHHwwhssTsnqw
            VpbpZZbvPLbZbbBhwqMHhsGMnJdVwV
            mgQZJDLBJbbbcbgZClCSfWlrCjRjlDCR
            fSpwcVfzsztcSSWNNMbnMRqTvtTv
            mJFmGDDDhGhBJHCQddllqTvCllqTRRWNnMbT
            FdFDGdDDDhhHdZDjhDmpwSPVZszpwZsVgsPRZs
            ";

        #endregion Input
    }
}
