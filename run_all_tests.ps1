$toolchains = @("+stable", @(,@())), @("+beta", @(,@())), @("+nightly", @(@("incomplete"), @()))
$features = "usize", "isize", "i8", "i16", "i32", "i64", "i128", "u8", "u16", "u32", "u64", "u128", "char", "bool"

foreach ($toolchain in $toolchains)
{
    foreach ($feature in $features)
    {
        foreach ($flag in $toolchain[1])
        {
            $f = @($feature) + $flag
            $f = $f -Join ","
            echo $toolchain[0] $f
            cargo $toolchain[0] "test" "--no-default-features" "--features" $f
            if (-not$?)
            {
                echo "ERROR:"
                echo $toolchain[0] $f
                exit
            }
        }
    }
}

foreach ($toolchain in $toolchains)
{
    foreach ($flag in $toolchain[1])
    {
        $f = $features + $flag
        $f = $f -Join ","
        echo $toolchain[0] $f
        cargo $toolchain[0] "test" "--no-default-features" "--features" $f
        if (-not$?)
        {
            echo "ERROR:"
            echo $toolchain[0] $f
            exit
        }
    }
}
exit
cargo "+nightly" "build" "--no-default-features"
if (-not$?)
{
    echo "ERROR:"
    exit
}

foreach ($i in 1..(1 -shl $features.Length))
{
    $current = @()
    foreach ($j in 0..$features.Length)
    {
        if (($i -shr $j) -band 1)
        {
            $current += $features[$j]
        }
    }
    $f = $current -join ","
    echo $f
    cargo "+nightly" "build" "--no-default-features" "--features" $f
    if (-not$?)
    {
        echo "ERROR:"
        exit
    }
}
