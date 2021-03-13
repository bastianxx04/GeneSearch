alphabet1 = ['$', 'A', 'C', 'G', 'T']
alphabet2 = ['$', 'G', 'L', 'O']
full_string = "AGATAGATTCACA$"
full_string_rev = full_string[::-1]
full_string2 = "AGATAGATTCACA$"
full_string2_rev = full_string[::-1]

# Test D stuff
string_googol = "GOOGOL$"
string_googol_rev = string_googol[::-1]


class O_Table():
    def __init__(self, table, bwt):
        self.table = table
        self.bwt = bwt

    def __getitem__(self, ci):
        c, i = ci
        for (char, lst) in self.table:
            if char == c:
                return lst[i]

    def __str__(self):
        ret_string = "O Table \n \n"
        ret_string += '{:>4}'.format(
            "") + "".join(['{:>4}'.format(c) for c in self.bwt]) + "\n"
        for (c, lst) in self.table:
            ret_string += '{:>4}'.format(c) + \
                "".join(['{:>4}'.format(n) for n in lst]) + "\n"
        return ret_string


class C_table():
    def __init__(self, table, alphabet):
        self.table = table
        self.alphabet = alphabet

    def __getitem__(self, c):
        idx = self.alphabet.index(c)
        return self.table[idx]

    def __str__(self):
        ret_string = "C Table \n \n"
        ret_string += "".join(['{:>4}'.format(a)
                               for a in self.alphabet]) + "\n"
        ret_string += "".join(['{:>4}'.format(n) for n in self.table]) + "\n"
        return ret_string


class D_table():
    """docstring for D_table"""

    def __init__(self, table, string):
        self.table = table
        self.string = string

    def __getitem__(self, key):
        return self.table[key]

    def __str__(self):
        ret_string = "D Table \n \n"
        ret_string += "".join(['{:>4}'.format(c) for c in self.string]) + "\n"
        ret_string += "".join(['{:>4}'.format(n) for n in self.table]) + "\n"
        return ret_string


def create_suffix_array(s):
    suffix = []
    for i in range(0, len(s)):
        suffix.append(str(s[i:len(s)] + s[0:i]))
    suffix.sort()
    print("Suffixes: ", suffix)
    return_str = [x[len(x) - 1] for x in suffix]
    print("BWT: ", return_str)
    return return_str


def create_c_table(bwt, alphabet):
    c_table = []
    for i, c in enumerate(alphabet):
        counter = 0
        for t in range(i - 1, -1, -1):
            counter += bwt.count(alphabet[t])
        c_table.append(counter)
    return C_table(c_table, alphabet)


def create_o_table(bwt, alphabet):
    o_table = []
    for c in alphabet:
        char_lst = []
        curr = 0
        for b in bwt:
            if b == c:
                curr += 1
            char_lst.append(curr)
        o_table.append((c, char_lst))
    return O_Table(o_table, bwt)


def calculate_d(W, bwt, O, C):
    D = []
    k = 1
    l = len(bwt) - 1
    z = 0
    for i in range(0, len(W)):
        k = C[W[i]] + O[W[i], k - 1]
        l = C[W[i]] + O[W[i], l] - 1
        if k > l:
            k = 1
            l = len(bwt) - 1
            z += 1
        D.append(z)
    return D_table(D, W)


def inex_recur(C, O, D, A, W, i, edits_left, L, R):
    #print(f"entered recur with level {i} and {D[-1]}")
    
    lower_limit = D[i]

    if edits_left < lower_limit:
        #print(f"    returned nothing")
        return set()

    if i < 0:
        #print(f"    returned something")
        return {(L, R)}
    
    I = set()
    I = I.union(inex_recur(C, O, D, A, W, i - 1, edits_left - 1, L, R))
    
    for b in A:

        #print(f"in loop: start: {L} - end: {R}")
        L = C[b] + O[b, L]
        R = C[b] + O[b, R]
        #print(f"after mathing: start: {L} - end: {R}")
        if L <= R:
            I = I.union(inex_recur(C, O, D, A, W, i, edits_left - 1, L, R))
            if b == W[i]:
                I = I.union(inex_recur(C, O, D, A, W, i - 1, edits_left, L, R))
            else:
                I = I.union(inex_recur(C, O, D, A, W, i - 1, edits_left - 1, L, R))
    return I


def run(string, alphabet):
    search_word = "ATT"
    print("String: ", string)
    bwt = create_suffix_array(string)
    c_table = create_c_table(bwt, alphabet)
    o_table = create_o_table(bwt, alphabet)
    print(o_table)
    print(c_table)
    d_table = calculate_d(search_word, bwt, o_table, c_table)
    print(d_table)
    result = inex_recur(c_table, o_table, d_table, alphabet, search_word, len(search_word)-1, 1, 1, len(string)-1)
    print(result)


run(full_string2, alphabet1)
