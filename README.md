# Images To VP9

把一些连续的图像转换成fMP4 with vp9，项目依赖libvpx.

vpx-encode 基于库 https://github.com/astraw/vpx-encode 上进行了一些修改，添加了一些libvpx的调用参数和支持yuv420p格式的输入。