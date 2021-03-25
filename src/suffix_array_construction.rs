use std::usize;

use crate::types::SuffixArray;
use crate::ALPHABET;

//SA-IS
/*
def buildTypeMap(data):
    """
    Builds a map marking each suffix of the data as S_TYPE or L_TYPE.
    """
    # The map should contain one more entry than there are characters
    # in the string, because we also need to store the type of the
    # empty suffix between the last character and the end of the
    # string.
    res = bytearray(len(data) + 1)
    # The empty suffix after the last character is S_TYPE
    res[-1] = S_TYPE
    # If this is an empty string...
    if not len(data):
        # ...there are no more characters, so we're done.
        return res
    # The suffix containing only the last character must necessarily
    # be larger than the empty suffix.
    res[-2] = L_TYPE
    # Step through the rest of the string from right to left.
    for i in range(len(data)-2, -1, -1):
        if data[i] > data[i+1]:
            res[i] = L_TYPE
        elif data[i] == data[i+1] and res[i+1] == L_TYPE:
            res[i] = L_TYPE
        else:
            res[i] = S_TYPE
    return res

def isLMSChar(offset, typemap):
    """
    Returns true if the character at offset is a left-most S-type.
    """
    if offset == 0:
        return False
    if typemap[offset] == S_TYPE and typemap[offset - 1] == L_TYPE:
        return True

    return False

def lmsSubstringsAreEqual(string, typemap, offsetA, offsetB):
    """
    Return True if LMS substrings at offsetA and offsetB are equal.
    """
    # No other substring is equal to the empty suffix.
    if offsetA == len(string) or offsetB == len(string):
        return False
    i = 0
    while True:
        aIsLMS = isLMSChar(i + offsetA, typemap)
        bIsLMS = isLMSChar(i + offsetB, typemap)
        # If we've found the start of the next LMS substrings...
        if (i > 0 and aIsLMS and bIsLMS):
            # ...then we made it all the way through our original LMS
            # substrings without finding a difference, so we can go
            # home now.
            return True
        if aIsLMS != bIsLMS:
            # We found the end of one LMS substring before we reached
            # the end of the other.
            return False
        if string[i + offsetA] != string[i + offsetB]:
            # We found a character difference, we're done.
            return False
        i += 1

def findBucketSizes(string, alphabetSize=256):
    res = [0] * alphabetSize
    for char in string:
        res[char] += 1
    return res


def findBucketHeads(bucketSizes):
    offset = 1
    res = []
    for size in bucketSizes:
        res.append(offset)
        offset += size

    return res


def findBucketTails(bucketSizes):
    offset = 1
    res = []
    for size in bucketSizes:
        offset += size
        res.append(offset - 1)

    return res

def makeSuffixArrayByInducedSorting(string, alphabetSize):
    """
    Compute the suffix array of 'string' with the SA-IS algorithm.
    """
    # Classify each character of the string as S_TYPE or L_TYPE
    typemap = buildTypeMap(string)
    # We'll be slotting suffixes into buckets according to what
    # character they start with, so let's precompute that info now.
    bucketSizes = findBucketSizes(string, alphabetSize)
    # Use a simple bucket-sort to insert all the LMS suffixes into
    # approximately the right place the suffix array.
    guessedSuffixArray = guessLMSSort(string, bucketSizes, typemap)
    # Slot all the other suffixes into guessedSuffixArray, by using
    # induced sorting. This may move the LMS suffixes around.
    induceSortL(string, guessedSuffixArray, bucketSizes, typemap)
    induceSortS(string, guessedSuffixArray, bucketSizes, typemap)
    # Create a new string that summarises the relative order of LMS
    # suffixes in the guessed suffix array.
    summaryString, summaryAlphabetSize, summarySuffixOffsets = \
        summariseSuffixArray(string, guessedSuffixArray, typemap)
    # Make a sorted suffix array of the summary string.
    summarySuffixArray = makeSummarySuffixArray(
        summaryString,
        summaryAlphabetSize,
    )
    # Using the suffix array of the summary string, determine exactly
    # where the LMS suffixes should go in our final array.
    result = accurateLMSSort(string, bucketSizes, typemap,
            summarySuffixArray, summarySuffixOffsets)
    # ...and once again, slot all the other suffixes into place with
    # induced sorting.
    induceSortL(string, result, bucketSizes, typemap)
    induceSortS(string, result, bucketSizes, typemap)
    return result

def guessLMSSort(string, bucketSizes, typemap):
    """
    Make a suffix array with LMS-substrings approximately right.
    """
    # Create a suffix array with room for a pointer to every suffix of
    # the string, including the empty suffix at the end.
    guessedSuffixArray = [-1] * (len(string) + 1)
    bucketTails = findBucketTails(bucketSizes)
    # Bucket-sort all the LMS suffixes into their appropriate bucket.
    for i in range(len(string)):
        if not isLMSChar(i, typemap):
            # Not the start of an LMS suffix
            continue
        # Which bucket does this suffix go into?
        bucketIndex = string[i]
        # Add the start position at the tail of the bucket...
        guessedSuffixArray[bucketTails[bucketIndex]] = i
        # ...and move the tail pointer down.
        bucketTails[bucketIndex] -= 1
        # Show the current state of the array
        showSuffixArray(guessedSuffixArray)
    # The empty suffix is defined to be an LMS-substring, and we know
    # it goes at the front.
    guessedSuffixArray[0] = len(string)
    showSuffixArray(guessedSuffixArray)
    return guessedSuffixArray

def induceSortL(string, guessedSuffixArray, bucketSizes, typemap):
    """
    Slot L-type suffixes into place.
    """
    bucketHeads = findBucketHeads(bucketSizes)
    # For each cell in the suffix array....
    for i in range(len(guessedSuffixArray)):
        if guessedSuffixArray[i] == -1:
            # No offset is recorded here.
            continue
        # We're interested in the suffix that begins to the left of
        # the suffix this entry points at.
        j = guessedSuffixArray[i] - 1
        if j < 0:
            # This entry in the suffix array is the suffix that begins
            # at the start of the string, offset 0. Therefore there is
            # no suffix to the left of it, and j is out of bounds of
            # the typemap.
            continue
        if typemap[j] != L_TYPE:
            # We're only interested in L-type suffixes right now.
            continue
        # Which bucket does this suffix go into?
        bucketIndex = string[j]
        # Add the start position at the head of the bucket...
        guessedSuffixArray[bucketHeads[bucketIndex]] = j
        # ...and move the head pointer up.
        bucketHeads[bucketIndex] += 1
        showSuffixArray(guessedSuffixArray, i)

def induceSortS(string, guessedSuffixArray, bucketSizes, typemap):
    """
    Slot S-type suffixes into place.
    """
    bucketTails = findBucketTails(bucketSizes)

    for i in range(len(guessedSuffixArray)-1, -1, -1):
        j = guessedSuffixArray[i] - 1
        if j < 0:
            # This entry in the suffix array is the suffix that begins
            # at the start of the string, offset 0. Therefore there is
            # no suffix to the left of it, and j is out of bounds of
            # the typemap.
            continue
        if typemap[j] != S_TYPE:
            # We're only interested in S-type suffixes right now.
            continue

        # Which bucket does this suffix go into?
        bucketIndex = string[j]
        # Add the start position at the tail of the bucket...
        guessedSuffixArray[bucketTails[bucketIndex]] = j
        # ...and move the tail pointer down.
        bucketTails[bucketIndex] -= 1

        showSuffixArray(guessedSuffixArray, i)

def summariseSuffixArray(string, guessedSuffixArray, typemap):
    """
    Construct a 'summary string' of the positions of LMS-substrings.
    """
    # We will use this array to store the names of LMS substrings in
    # the positions they appear in the original string.
    lmsNames = [-1] * (len(string) + 1)
    # Keep track of what names we've allocated.
    currentName = 0
    # Where in the original string was the last LMS suffix we checked?
    lastLMSSuffixOffset = None
    # We know that the first LMS-substring we'll see will always be
    # the one representing the empty suffix, and it will always be at
    # position 0 of suffixOffset.
    lmsNames[guessedSuffixArray[0]] = currentName
    lastLMSSuffixOffset = guessedSuffixArray[0]
    showSuffixArray(lmsNames)
    # For each suffix in the suffix array...
    for i in range(1, len(guessedSuffixArray)):
        # ...where does this suffix appear in the original string?
        suffixOffset = guessedSuffixArray[i]
        # We only care about LMS suffixes.
        if not isLMSChar(suffixOffset, typemap):
            continue
        # If this LMS suffix starts with a different LMS substring
        # from the last suffix we looked at....
        if not lmsSubstringsAreEqual(string, typemap,
                lastLMSSuffixOffset, suffixOffset):
            # ...then it gets a new name
            currentName += 1
        # Record the last LMS suffix we looked at.
        lastLMSSuffixOffset = suffixOffset
        # Store the name of this LMS suffix in lmsNames, in the same
        # place this suffix occurs in the original string.
        lmsNames[suffixOffset] = currentName
        showSuffixArray(lmsNames)
    # Now lmsNames contains all the characters of the suffix string in
    # the correct order, but it also contains a lot of unused indexes
    # we don't care about and which we want to remove. We also take
    # this opportunity to build summarySuffixOffsets, which tells
    # us which LMS-suffix each item in the summary string represents.
    # This will be important later.
    summarySuffixOffsets = []
    summaryString = []
    for index, name in enumerate(lmsNames):
        if name == -1:
            continue
        summarySuffixOffsets.append(index)
        summaryString.append(name)
    # The alphabetically smallest character in the summary string
    # is numbered zero, so the total number of characters in our
    # alphabet is one larger than the largest numbered character.
    summaryAlphabetSize = currentName + 1
    return summaryString, summaryAlphabetSize, summarySuffixOffsets

def makeSummarySuffixArray(summaryString, summaryAlphabetSize):
    """
    Construct a sorted suffix array of the summary string.
    """
    if summaryAlphabetSize == len(summaryString):
        # Every character of this summary string appears once and only
        # once, so we can make the suffix array with a bucket sort.
        summarySuffixArray = [-1] * (len(summaryString) + 1)

        # Always include the empty suffix at the beginning.
        summarySuffixArray[0] = len(summaryString)

        for x in range(len(summaryString)):
            y = summaryString[x]
            summarySuffixArray[y+1] = x

    else:
        # This summary string is a little more complex, so we'll have
        # to use recursion.
        summarySuffixArray = makeSuffixArrayByInducedSorting(
            summaryString,
            summaryAlphabetSize,
        )

    return summarySuffixArray

def accurateLMSSort(string, bucketSizes, typemap,
        summarySuffixArray, summarySuffixOffsets):
    """
    Make a suffix array with LMS suffixes exactly right.
    """
    # A suffix for every character, plus the empty suffix.
    suffixOffsets = [-1] * (len(string) + 1)

    # As before, we'll be adding suffixes to the ends of their
    # respective buckets, so to keep them in the right order we'll
    # have to iterate through summarySuffixArray in reverse order.
    bucketTails = findBucketTails(bucketSizes)
    for i in range(len(summarySuffixArray)-1, 1, -1):
        stringIndex = summarySuffixOffsets[summarySuffixArray[i]]

        # Which bucket does this suffix go into?
        bucketIndex = string[stringIndex]
        # Add the suffix at the tail of the bucket...
        suffixOffsets[bucketTails[bucketIndex]] = stringIndex
        # ...and move the tail pointer down.
        bucketTails[bucketIndex] -= 1

        showSuffixArray(suffixOffsets)

    # Always include the empty suffix at the beginning.
    suffixOffsets[0] = len(string)

    showSuffixArray(suffixOffsets)

    return suffixOffsets

def makeSuffixArray(bytestring):
    return makeSuffixArrayByInducedSorting(bytestring, 256)

*/

/// SA-IS
#[allow(dead_code)]
pub fn suffix_array_induced_sort(reference: &[u8]) -> SuffixArray {
    /*
    t: array [0..n − 1] of boolean;
    P1: array [0..n_1 − 1] of integer;
    S1: array [0..n_1 − 1] of integer;
    B: array [0..∥Σ(S)∥ − 1] of integer;

    Scan S once to classify all the characters as L- or S-type into t;
    Scan t once to find all the LMS-substrings in S into P1;
    Induced sort all the LMS-substrings using P1 and B;
    Name each LMS-substring in S by its bucket index to get a new shortened string S1;
    if Each character in S1 is unique
        then
            Directly compute SA1 from S1;
        else
            SA-IS(S1, SA1); � where recursive call happens
    Induce SA from SA1;
    return
    */
    let n = reference.len();
    let t = build_type_array(reference);
    let lms_pointers = build_lms_array(&t);

    let bucket_sizes = build_bucket_sizes(reference);
    let bucket_heads = find_bucket_heads(&bucket_sizes);
    let bucket_tails = find_bucket_tails(&bucket_sizes);
    
    let mut suffix_array = vec![usize::MAX; n];
    place_lms(&mut suffix_array, reference, &lms_pointers, bucket_tails.clone());
    induce_l_types(&mut suffix_array, reference, &t, bucket_heads.clone());
    induce_s_types(&mut suffix_array, reference, &t, bucket_tails.clone());
    
    // step 4: cry
    //let mut reduced_reference = Vec::new();
    // For hver LMS substring, find dens bucket index og læg ind i reduced_reference

    for i in lms_pointers {
        for j in 0..ALPHABET.len() {
            if i > bucket_tails[j] && i < bucket_heads[j] {
                
            }
        }
    }
    
    suffix_array
}

fn build_type_array(reference: &[u8]) -> Vec<bool> {
    let n = reference.len();
    let mut type_map = vec![false; n];
    type_map[n - 1] = true;

    for i in (0..n - 1).rev() {
        if reference[i] == reference[i + 1] {
            type_map[i] = type_map[i + 1];
        } else {
            type_map[i] = reference[i] < reference[i + 1];
        }
    }

    type_map
}

fn build_lms_array(t: &[bool]) -> Vec<usize> {
    let n = t.len();
    let mut lms_substrings = Vec::new();

    if t[0] {
        lms_substrings.push(0)
    }
    for i in 1..n {
        if t[i] && !t[i - 1] {
            lms_substrings.push(i)
        }
    }

    lms_substrings
}

fn build_bucket_sizes(reference: &[u8]) -> Vec<usize> {
    let mut bucket_sizes = vec![0; ALPHABET.len()];

    for &c in reference {
        bucket_sizes[c as usize] += 1;
    }

    bucket_sizes
}

fn find_bucket_heads(buckets: &[usize]) -> Vec<usize> {
    let mut offset = 1;
    let mut result = Vec::new();

    for size in buckets {
        result.push(offset);
        offset += size;
    }

    result
}

fn find_bucket_tails(buckets: &[usize]) -> Vec<usize> {
    let mut offset = 1;
    let mut result = Vec::new();

    for size in buckets {
        offset += size;
        result.push(offset - 1);
    }

    result
}

fn place_lms(suffix_array: &mut SuffixArray, reference: &[u8], lms_pointers: &[usize], mut bucket_tails: Vec<usize>) {
    for &i in lms_pointers {
        let c = reference[i] as usize;
        suffix_array[bucket_tails[c] - 1] = i;
        bucket_tails[c] -= 1;
    }
}

fn induce_l_types(suffix_array: &mut SuffixArray, reference: &[u8], t: &[bool], mut bucket_heads: Vec<usize>) {
    // STEP 2 (it's about to get crazy)
    let n = reference.len();
    for i in 0..n {
        if suffix_array[i] == usize::MAX || suffix_array[i] == 0 {
            continue;
        }

        let j = suffix_array[i] - 1;
        
        if !t[j] {
            let c = reference[j] as usize;
            suffix_array[bucket_heads[c] - 1] = j;
            bucket_heads[c] += 1;
        }
    }
}

fn induce_s_types(suffix_array: &mut SuffixArray, reference: &[u8], t: &[bool], mut bucket_tails: Vec<usize>) {
    // STEP 3 (the one where the magic happens)
    let n = reference.len();
    for i in (0..n).rev() {
        if suffix_array[i] == usize::MAX || suffix_array[i] == 0 {
            continue;
        }

        let j = suffix_array[i] - 1;
        if t[j] {
            let c = reference[j] as usize;
            suffix_array[bucket_tails[c] - 1] = j;
            bucket_tails[c] -= 1;
        }
    }
}

/// Construct a suffix array naively
pub fn construct_suffix_array_naive(reference: &[u8]) -> SuffixArray {
    let mut temp_data_table: Vec<(Vec<u8>, usize)> = Vec::new();
    for i in 0..(reference.len()) {
        let mut to_be_inserted = reference.to_owned();

        if !temp_data_table.is_empty() {
            to_be_inserted = temp_data_table.last().unwrap().0.clone();
            to_be_inserted.remove(0);
        }
        temp_data_table.push((to_be_inserted, i));
    }

    temp_data_table.sort();

    temp_data_table.iter().map(|elem| elem.1).collect()
}

#[cfg(test)]
mod tests {
    use crate::{read_genome, util::string_to_ints, HG38_1000_PATH};

    use super::*;
    use test::Bencher;

    #[test]
    fn test_type_map() {
        let reference = string_to_ints("ACATGA$");
        let t = build_type_array(&reference);
        assert_eq!(vec![true, false, true, false, false, false, true], t);
    }

    #[test]
    fn test_sais_mmiissiissiippii() {
        let reference = string_to_ints("CCAATTAATTAAGGAA$");
        let sa = suffix_array_induced_sort(&reference);
        assert_eq!(sa.len(), 99999999999999999);
    }

    #[test]
    fn test_sais_compare_naive() {
        let genome_string = "AATAAACCTTACCTAGCACTCCATCATGTCTTATGGCGCGTGATTTGCCCCGGACTCAGG$";
        let genome = string_to_ints(&genome_string);
        let naive = construct_suffix_array_naive(&genome);
        let sais = suffix_array_induced_sort(&genome);
        assert_eq!(naive, sais);
    }

    #[bench]
    #[ignore = "slow"]
    fn bench_sa_naive_ref1000(b: &mut Bencher) {
        let genome_string = read_genome(HG38_1000_PATH).unwrap();
        let genome = string_to_ints(&genome_string);
        b.iter(|| construct_suffix_array_naive(&genome))
    }

    #[bench]
    #[ignore = "slow"]
    fn bench_sais_ref1000(b: &mut Bencher) {
        let genome_string = read_genome(HG38_1000_PATH).unwrap();
        let genome = string_to_ints(&genome_string);
        b.iter(|| suffix_array_induced_sort(&genome))
    }
}
