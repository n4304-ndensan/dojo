import java.util.*;

public class Main {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        int n = sc.nextInt();
        int[] a = new int[n];
        for (int i = 0; i < n; i++) {
            a[i] = sc.nextInt();
        }

        long ans = sumOfPairProducts(a);
        System.out.println(ans);
    }

    private static long sumOfPairProducts(int[] products) {
        long sum = 0;
        for (int product : products) {
            sum += product;
        }

        long sumOfPairProducts = 0;
        for (int product : products) {
            sumOfPairProducts += product * (sum - product);
            sum -= product;
        }

        return sumOfPairProducts;
    }
}
