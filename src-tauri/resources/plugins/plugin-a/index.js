// 导入 Node.js 的内置模块
const process = require("process");

const app = () => {
  // 获取命令行参数
  const args = process.argv.slice(2);

  // 如果参数个数不为2，给出错误提示
  if (args.length !== 2) {
    console.log("请提供两个数字作为参数！");
    process.exit(1);
  }

  // 解析参数为数字
  const num1 = parseFloat(args[0]);
  const num2 = parseFloat(args[1]);

  // 检查参数是否为有效数字
  if (isNaN(num1) || isNaN(num2)) {
    console.log("参数必须为有效的数字！");
    process.exit(1);
  }

  // 计算两个数的和
  const sum = num1 + num2;
  // 输出结果
  return sum;
};

console.log(app());
