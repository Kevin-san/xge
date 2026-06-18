# Sprint 17 · 示例程序

> 上游 sprint: [Sprint 17](../sprint-17-math-simd.md)

---

## 示例列表

### 01-simd-bench
**功能：** SIMD vs scalar 性能对比
**输出：** 控制台表格

```rust
use engine_math::f32x4;

fn main() {
    let scalars: Vec<[f32; 4]> = (0..10_000_000).map(|i| [i as f32; 4]).collect();
    
    // scalar
    let start = std::time::Instant::now();
    let mut sum_scalar = 0.0;
    for v in &scalars {
        sum_scalar += v[0] * v[1] + v[2] * v[3];
    }
    let dt_scalar = start.elapsed();
    
    // SIMD
    let start = std::time::Instant::now();
    let mut sum_simd = 0.0;
    for chunk in scalars.chunks_exact(4) {
        let v0 = f32x4::from_array(chunk[0]);
        let v1 = f32x4::from_array(chunk[1]);
        let v2 = f32x4::from_array(chunk[2]);
        let v3 = f32x4::from_array(chunk[3]);
        let dot = v0.dot(v1) + v2.dot(v3);
        sum_simd += dot;
    }
    let dt_simd = start.elapsed();
    
    println!("Scalar: {:?}", dt_scalar);
    println!("SIMD:   {:?}", dt_simd);
    println!("Speedup: {:.2}x", dt_scalar.as_secs_f64() / dt_simd.as_secs_f64());
}
```

### 02-mat4-inverse-test
**功能：** Mat4 inverse 1000 个随机矩阵交叉验证
**输出：** 验证结果 + 误差

### 03-quat-squad-demo
**功能：** 球面四边形插值 4 关键帧
**输出：** 角度曲线图

### 04-dual-quat-skinning
**功能：** 4 关节机械臂 DLB vs LBS 视觉对比
**输出：** 终端位置坐标序列

### 05-frustum-cull
**功能：** 10000 物体视锥剔除
**输出：** 帧时间 + culled count
