use biot::{neighbors, approx_pattern_count};

pub fn motif_enumeration(dna: Vec<String>, k: i32, d: i32) -> Vec<String> {
    let mut patterns: Vec<String> = vec!();
    let k_mers = find_k_mers(dna.clone(), k);

    for k_mer in k_mers {
        let k_neighbors = neighbors(&k_mer, d);
        for k_neighbor in k_neighbors {
            if dna.iter().fold(true, |acc, line| acc && approx_pattern_count(&line, &k_neighbor, d) >= 1) && !patterns.contains(&k_neighbor) {
                patterns.push(k_neighbor);
            }
        }
    }
    patterns
}

fn find_k_mers(dna: Vec<String>, k: i32) -> Vec<String> {
    let mut k_mers: Vec<String> = vec!();

    for line in dna {
        for i in 0..(line.len() - k as usize + 1) {
            let k_mer = &line[i..(i + k as usize)];
            k_mers.push(String::from(k_mer));
        }
    }

    k_mers
}

fn find_consensus(dna: Vec<String>, k: i32) -> Vec<usize> {
    let mut consensus_matrix = vec![vec![0; k as usize]; 4];
    
    let k_mers = find_k_mers(dna.clone(), k);

    for k_mer in k_mers {
        for i in 0..k_mer.len() {
            let token = &k_mer[i..i+1];
            let nucleotide_index = match token {
                "A" => 0,
                "C" => 1,
                "G" => 2,
                "T" => 3,
                _ => unreachable!()
            };

            consensus_matrix[nucleotide_index][i] += 1;
        }
    }

    let mut results = vec!();

    for i in 0..k {
        let mut max_nucleotide = 4;
        let mut current_max = 0;
        for n in 0..4 {
            let n_count = consensus_matrix[n][i as usize];
            if n_count > current_max {
                max_nucleotide = n;
                current_max = n_count;
            }
        }

        results.push(max_nucleotide);
    }

    results
}

#[cfg(test)]
mod motif_enumeration_test {
    use super::motif_enumeration;

    #[test]
    fn it_passes_sample_input() {
        let dna_lines = vec!(
            String::from("ATTTGGC"),
            String::from("TGCCTTA"),
            String::from("CGGTATC"),
            String::from("GAAAATT")
        );

        let result = motif_enumeration(dna_lines, 3, 1);

        assert_eq!(result.len(), 4);
        assert!(result.contains(&String::from("ATA")));
        assert!(result.contains(&String::from("ATT")));
        assert!(result.contains(&String::from("GTT")));
        assert!(result.contains(&String::from("TTT")));

    }

    #[test]
    fn it_passes_extra_input() {
        let dna_lines = vec!(
                            String::from("TCTGAGCTTGCGTTATTTTTAGACC"),
                            String::from("GTTTGACGGGAACCCGACGCCTATA"),
                            String::from("TTTTAGATTTCCTCAGTCCACTATA"),
                            String::from("CTTACAATTTCGTTATTTATCTAAT"),
                            String::from("CAGTAGGAATAGCCACTTTGTTGTA"),
                            String::from("AAATCCATTAAGGAAAGACGACCGT"),

                            );

        let output_str = "AAACT AAATC AACAC AACAT AACCT AACTA AACTC AACTG AACTT AAGAA AAGCT AAGGT AAGTC AATAC AATAT AATCC AATCT AATGC AATTC AATTG ACAAC ACACA ACACC ACACG ACACT ACAGA ACAGC ACATC ACATG ACCAT ACCCT ACCGT ACCTA ACCTC ACCTG ACCTT ACGAC ACGAG ACGAT ACGCT ACGGT ACGTC ACGTT ACTAA ACTAG ACTAT ACTCA ACTCC ACTCG ACTCT ACTGA ACTGC ACTGT ACTTA ACTTC ACTTT AGAAA AGAAC AGAAG AGAAT AGACA AGACT AGATA AGATC AGCAT AGCCA AGCGT AGCTA AGCTC AGCTG AGCTT AGGAT AGGTA AGGTC AGTAA AGTAC AGTAT AGTCC AGTCG AGTCT AGTGA AGTTG ATAAA ATAAC ATACA ATACC ATAGA ATATA ATATC ATATG ATATT ATCAG ATCCC ATCCG ATCCT ATCGA ATCGC ATCTA ATCTC ATCTG ATGAC ATGAT ATGCA ATGCC ATGGA ATGGC ATGTA ATGTC ATTAA ATTAC ATTAG ATTAT ATTCA ATTCC ATTCG ATTGA ATTGC ATTGG ATTGT ATTTA ATTTC ATTTG ATTTT CAAAG CAACC CAACT CAAGA CAAGC CAATA CAATT CACAC CACAG CACCT CACGT CACTA CACTT CAGAA CAGAC CAGAT CAGGT CAGTA CAGTC CATAA CATAC CATAG CATAT CATCC CATCT CATGA CATGT CATTA CATTG CATTT CCAAG CCATA CCATG CCATT CCCGT CCCTA CCCTT CCGAA CCGAC CCGAT CCGCT CCGGT CCGTA CCGTC CCGTG CCGTT CCTAC CCTAT CCTCA CCTCC CCTTA CCTTC CCTTG CCTTT CGAAA CGAAG CGACA CGACT CGAGT CGATA CGATG CGATT CGCAA CGCAT CGCCA CGCGA CGCTA CGCTC CGCTT CGGAC CGGAT CGGCA CGGTA CGGTC CGGTT CGTAA CGTAC CGTCA CGTCG CGTCT CGTTA CGTTT CTAAC CTAAG CTAAT CTACA CTACC CTACG CTACT CTAGA CTAGC CTAGG CTAGT CTATA CTATC CTATG CTATT CTCAT CTCCG CTCGT CTCTA CTCTT CTGAA CTGAG CTGCA CTGCC CTGTA CTGTT CTTAA CTTAC CTTAG CTTAT CTTCA CTTGA CTTTA CTTTC CTTTG CTTTT GAAAT GAACA GAACT GAAGT GAATG GAATT GACAC GACAT GACCA GACCT GACGT GACTT GAGAA GAGAT GAGCT GATAA GATAC GATAG GATAT GATCA GATCC GATCG GATCT GATGT GATTA GATTC GATTG GATTT GCAAT GCACT GCATC GCATT GCCAT GCCGT GCCTA GCCTT GCGAT GCGGT GCGTC GCGTT GCTAA GCTAC GCTAG GCTAT GCTGA GCTGT GCTTA GCTTT GGAAT GGACA GGATA GGATC GGATT GGCTA GGGAT GGTAC GGTAG GGTAT GGTCA GGTCG GGTTA GTAAA GTAAG GTACA GTACC GTACG GTAGA GTATA GTATC GTATG GTATT GTCAA GTCAG GTCCG GTCCT GTCGA GTCGC GTCGT GTCTA GTCTG GTGAA GTGAG GTGCA GTGCG GTTAA GTTAC GTTAG GTTAT GTTCA GTTCC GTTCG GTTGA GTTTA TAAAC TAAAG TAACA TAACC TAACT TAAGA TAAGC TAATA TAATC TACAC TACAG TACCC TACCG TACCT TACGA TACGC TACGT TACTA TACTC TACTG TAGAA TAGAC TAGAG TAGAT TAGCC TAGCG TAGGA TAGTC TATAA TATAC TATAT TATCA TATCC TATCG TATGA TATGC TATGG TATGT TATTA TATTG TCAAC TCAAT TCACC TCACG TCACT TCAGA TCATA TCATG TCCAA TCCAC TCCAG TCCAT TCCCA TCCCT TCCGA TCCGC TCCGT TCCTA TCCTG TCCTT TCGAA TCGAC TCGAT TCGCC TCGCT TCGGA TCGGC TCGGG TCGGT TCGTC TCTAC TCTAG TCTAT TCTCC TCTCT TCTGG TCTGT TCTTA TCTTT TGAAA TGAAC TGAAT TGACA TGACC TGACT TGAGA TGAGC TGAGT TGATA TGATC TGATG TGATT TGCAA TGCAC TGCAG TGCAT TGCCA TGCCG TGCCT TGCGA TGCGT TGCTT TGGAA TGGAT TGGTA TGTAA TGTAG TGTAT TGTCC TGTCG TGTGG TGTTA TTAAA TTAAC TTAAG TTAAT TTACA TTACC TTACG TTACT TTAGA TTAGC TTAGG TTAGT TTATA TTATC TTATG TTATT TTCAA TTCAC TTCAT TTCCA TTCCC TTCCT TTCGA TTCGG TTCGT TTCTA TTCTG TTGAA TTGAC TTGAG TTGAT TTGCA TTGCG TTGGA TTGGG TTGTG TTTAA TTTAC TTTAG TTTAT TTTCA TTTCC TTTCG TTTGA TTTGG TTTTA TTTTG";

        let output: Vec<&str> = output_str.split(' ').collect();

        let result = motif_enumeration(dna_lines, 5, 2);

        assert_eq!(result.len(), output.len());

        assert!(output.iter().fold(true, |acc, &val| acc && result.contains(&String::from(val))));

    }
}
