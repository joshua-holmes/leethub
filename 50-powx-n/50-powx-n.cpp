class Solution {
public:
    double myPow(double x, int n) {
        // if (n == 0) return 1;
        // int expo = abs(n);
        // double result = n < 0 ? 1/x : x;
        // double mult = result;
        // for (int i = 1; i < expo; i++) {
        //     result *= mult;
        // }
        // return result;
        double ans=1;
        int temp=abs(n);
        while(temp>0){
            int last_digit=temp&1;
            if(last_digit) ans*=x;
            x*=x;
            temp>>=1;
        }
        if(n>0)
        return ans;
        else return(1.0/ans);
    }
};