class Solution {
    public int countStudents(int[] sts, int[] sds) {
        int ones = 0;
        int zeros = 0;

        for(int s : sts){
            if(s == 0) zeros++;
            else ones++;
        }

        for(int s : sds){
            if(s == 1){
                if(ones == 0) return zeros;
                ones--;
            } else {
                if(zeros == 0) return ones;
                zeros--;
            }
        }

        return ones + zeros;
    }
}