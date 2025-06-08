class Solution {
    public int findKthLargest(int[] nums, int k) {
        sort(nums);
        return nums[k - 1];
    }

    void sort(int[] arr){
        int[] tmp = new int[arr.length];
        mergeSort(arr, tmp, 0, arr.length - 1);
    }

    void mergeSort(int[] arr, int[] tmp, int l, int r){
        if(l >= r) return;

        int m = l + (r - l) / 2;

        mergeSort(arr, tmp, l, m);
        mergeSort(arr, tmp, m + 1, r);  
        merge(arr, tmp, l, m, r);
    }

    void merge(int[] arr, int[] tmp, int l, int m, int r){
        int i = l;
        int j = m + 1;
        int len = l;

        while(i <= m && j <= r){
            if(arr[i] > arr[j]) tmp[len++] = arr[i++];
            else tmp[len++] = arr[j++];
        }

        while(i <= m) tmp[len++] = arr[i++];
        while(j <= r) tmp[len++] = arr[j++];

        for(int k = l; k <= r; k++){
            arr[k] = tmp[k];
        }
    }
}