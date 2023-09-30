

class Solution {
public:
    string reverseVowels(string s) {
        int left = 0;
        int right = s.size() - 1;
        
        while (left < right) {
            bool leftIsVowel = false;
            bool rightIsVowel = false;
            string vowels = "aeiouAEIOU";
            for (char v : vowels) {
                if (s[left] == v) {
                    leftIsVowel = true;
                }
                if (s[right] == v) {
                    rightIsVowel = true;
                }
                if (leftIsVowel && rightIsVowel) {
                    break;
                }
            }
            if (!leftIsVowel)
                left++;
            if (!rightIsVowel)
                right--;
            if (leftIsVowel && rightIsVowel) {
                char rightV = s[right];
                s[right] = s[left];
                s[left] = rightV;
                left++;
                right--;
            }
        }
        
        return s;
    }
};

