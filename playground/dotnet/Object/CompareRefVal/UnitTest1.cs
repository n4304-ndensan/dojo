using Xunit.Abstractions;

public class UnitTest1
{
    private ITestOutputHelper _helper;
    // TestOutputHelper
    public UnitTest1(ITestOutputHelper helper)
    {
        _helper = helper;
    }

    [Fact]
    public void RefTest()
    {
        var target = new ReferenceTypeMethodTest(_helper);
        target.Main();
    }

    [Fact]
    public void RefWithNewTest()
    {
        var target = new ReferenceTypeMethodTest(_helper);
        target.MainWithNew();
    }

    [Fact]
    public void ValTest()
    {
        var target = new ValueTypeMethodTest(_helper);
        target.Main();
    }
}

class PointClass
{
    public int x, y;

    public PointClass(int x, int y)
    {
        this.x = x;
        this.y = y;
    }

    public override string ToString()
    {
        return $"({x}, {y})";
    }
}

class ReferenceTypeMethodTest
{
    private ITestOutputHelper _helper;

    public ReferenceTypeMethodTest(ITestOutputHelper helper)
    {
        _helper = helper;
    }

    static void EditWithNew(PointClass p)
    {
        // メソッド内のローカル変数に新しいインスタンスを代入
        p = new PointClass(0, 0);

        // ローカル変数を編集
        p.x = 0;
        p.y = 0;
    }

    static void Edit(PointClass p)
    {
        // メソッド内のローカル変数に代入
        PointClass local = p;

        // ローカル変数を編集
        local.x = 0;
        local.y = 0;
    }

    public void Main()
    {
        _helper.WriteLine("=== 参照型（class）===");

        PointClass a = new PointClass(12, 5);
        _helper.WriteLine("before a = " + a);

        Edit(a);

        _helper.WriteLine("after  a = " + a);
    }

    public void MainWithNew()
    {
        _helper.WriteLine("=== 参照型（class） with new ===");

        PointClass a = new PointClass(12, 5);
        _helper.WriteLine("before a = " + a);

        EditWithNew(a);

        _helper.WriteLine("after  a = " + a);
    }
}


struct PointStruct
{
    public int x, y;

    public PointStruct(int x, int y)
    {
        this.x = x;
        this.y = y;
    }

    public override string ToString()
    {
        return $"({x}, {y})";
    }
}

class ValueTypeMethodTest
{
    private ITestOutputHelper _helper;
    public ValueTypeMethodTest(ITestOutputHelper helper)
    {
        _helper = helper;
    }

    static void Edit(PointStruct p)
    {
        // メソッド内のローカル変数に代入（ここでコピー）
        PointStruct local = p;

        // ローカル変数を編集
        local.x = 0;
        local.y = 0;
    }

    public void Main()
    {
        _helper.WriteLine("=== 値型（struct）===");

        PointStruct a = new PointStruct(12, 5);
        _helper.WriteLine("before a = " + a);

        Edit(a);

        _helper.WriteLine("after  a = " + a);
    }
}
