import java.util.Arrays;

class MergeSort {
    public static void main(String[] args) {
        MergeSort mergeSort = new MergeSort();

        int[] arr = new int[]{1, 3, 2, 5, 3, 7, 6, 4};
        int[] tmp = new int[arr.length];

        mergeSort.mergeSort(arr, tmp, 0, arr.length - 1);

        System.out.println(Arrays.toString(arr));
    }

    void mergeSort(int[] arr, int[] tmp, int left, int right){
        if(left >= right) return;
        int mid = left + (right - left) / 2;
        mergeSort(arr, tmp, left, mid);
        mergeSort(arr, tmp, mid + 1, right);
        merge(arr, tmp, left, mid, right);
    }

    void merge(int[] arr, int[] tmp, int left, int mid, int right){
        // left part
        int i = left;
        // right part
        int j = mid + 1;
        int k = left;

        while (i <= mid && j <= right){
            if(arr[i] < arr[j]){
                tmp[k++] = arr[i++];
            } else {
                tmp[k++] = arr[j++];
            }
        }

        while (i <= left) tmp[k++] = arr[i++];
        while (j <= right) tmp[k++] = arr[j++];

        for (int l = left; l <= right; l++) {
            arr[l] = tmp[l];
        }
    }
}