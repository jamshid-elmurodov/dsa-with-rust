class Solution {
    public void sortColors(int[] nums) {
        sort(nums);
    }

    void sort(int[] arr){
        int[] tmp = new int[arr.length];
        mergeSort(arr, tmp, 0, arr.length - 1);
    }

    void mergeSort(int[] arr, int[] tmp, int l, int r){
        if (l >= r) return;

        int mid = l + (r - l) / 2;
        mergeSort(arr, tmp, l, mid);
        mergeSort(arr, tmp, mid + 1, r);
        merge(arr, tmp, l, mid, r);
    }

    void merge(int[] arr, int[] tmp, int l, int mid, int r){
        int i = l;
        int j = mid + 1;
        int k = l;

        while(i <= mid && j <= r){
            if(arr[i] < arr[j]){
                tmp[k++] = arr[i++];
            } else {
                tmp[k++] = arr[j++];
            }
        }

        while(i <= mid) tmp[k++] = arr[i++];
        while(j <= r) tmp[k++] = arr[j++];

        for(int f = l; f <= r; f++){
            arr[f] = tmp[f];
        }
    }
}