"""
Michael Persico
Sept.29, 2021
Complementary DNA
https://www.codewars.com/kata/554e4a2f232cdd87d9000038
"""

def DNA_strand(dna):
    compl_strand = ""
    compl_nucleotide = {"A": "T", "T": "A", "G": "C", "C": "G"}
    [compl_strand := compl_strand + compl_nucleotide[nucleotide] for nucleotide in dna]
    return compl_strand
    
if __name__ == "__main__":
    print(DNA_strand("AAAA")) # "TTTT"
    print(DNA_strand("ATTGC")) # "TAACG"
    print(DNA_strand("GTAT")) # "CATA"
