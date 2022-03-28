class Solution:
    def isPalindrome(self, x: int) -> bool:
        # Turn number into string
        s = str(x)
        # Find beginning and end halves of .
        beg = s[:int(len(s)/2)]
        end = s[int(len(s)/2) + (len(s) % 2):]
        # Compare beginning and reversed end of string
        return beg == end[::-1]