public class Main {
    public static void main(String[] args) {
        // 入力
        java.util.Scanner sc = new java.util.Scanner(System.in);
        int n = sc.nextInt();
        // 2進数に変換
        String binary = Integer.toBinaryString(n);
        System.out.println(binary);
    }
}
