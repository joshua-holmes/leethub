

class Solution {
public:
    bool isVowel(char* letter) {
        string vowels = "aeiouAEIOU";
        for (char v : vowels) {
            if (*letter == v)
                return true;
        }

        return false;
    }
    string reverseVowels(string s) {
        int left = 0;
        int right = s.size() - 1;
        
        while (left < right) {
            bool leftIsVowel = isVowel(&s[left]);
            bool rightIsVowel = isVowel(&s[right]);
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

