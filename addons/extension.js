const vscode = require('vscode');
const { exec } = require('child_process');
const path = require('path');
const fs = require('fs');

/**
 * 激活扩展
 * @param {vscode.ExtensionContext} context
 */
function activate(context) {
    // 注册运行代码命令
    let disposable = vscode.commands.registerCommand('leonbasic.runCode', async function () {
        // 获取当前活动编辑器
        const editor = vscode.window.activeTextEditor;
        if (!editor) {
            vscode.window.showErrorMessage('请打开一个LeonBasic文件');
            return;
        }

        // 检查文件是否保存
        if (editor.document.isDirty) {
            await editor.document.save();
        }

        // 获取文件路径
        const filePath = editor.document.uri.fsPath;
        
        // 让用户选择leonlang可执行文件
        const options = {
            title: '选择LeonBasic可执行文件',
            filters: {
                '可执行文件': ['exe'],
                '所有文件': ['*']
            },
            canSelectMany: false
        };
        
        const result = await vscode.window.showOpenDialog(options);
        if (!result || result.length === 0) {
            vscode.window.showInformationMessage('未选择LeonBasic可执行文件');
            return;
        }
        
        const leonlangPath = result[0].fsPath;

        // 显示输出通道
        const outputChannel = vscode.window.createOutputChannel('LeonBasic');
        outputChannel.show();

        try {
            // 运行代码
            const command = `"${leonlangPath}" "${filePath}"`;
            outputChannel.appendLine(`正在运行: ${command}`);
            outputChannel.appendLine('---');
            
            console.log(`执行命令: ${command}`);

            // 执行命令
            try {
                const { stdout, stderr } = await new Promise((resolve, reject) => {
                    exec(command, (error, stdout, stderr) => {
                        if (error) {
                            reject({ error, stdout, stderr });
                        } else {
                            resolve({ stdout, stderr });
                        }
                    });
                });
                
                if (stdout) {
                    console.log('标准输出:', stdout);
                    outputChannel.append(stdout);
                }
                if (stderr) {
                    console.log('标准错误:', stderr);
                    outputChannel.appendLine('错误:');
                    outputChannel.append(stderr);
                }
                
                outputChannel.appendLine('---');
                outputChannel.appendLine('执行完成');
            } catch (err) {
                console.log('执行错误:', err.error ? err.error.message : err.message);
                if (err.stdout) {
                    outputChannel.append(err.stdout);
                }
                if (err.stderr) {
                    outputChannel.appendLine('错误:');
                    outputChannel.append(err.stderr);
                }
                outputChannel.appendLine(`执行失败: ${err.error ? err.error.message : err.message}`);
            }
        } catch (err) {
            outputChannel.appendLine(`运行错误: ${err.message}`);
            vscode.window.showErrorMessage(`运行LeonBasic代码时出错: ${err.message}`);
        }
    });

    context.subscriptions.push(disposable);
}

/**
 * 清理之前的findLeonlangExecutable函数，现在使用用户选择对话框替代
 */

// 导出激活函数
module.exports = {
    activate,
    deactivate
};

/**
 * 停用扩展
 */
function deactivate() {
    // 清理资源（如果需要）
}