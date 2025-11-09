const vscode = require('vscode');
const { exec } = require('child_process');
const path = require('path');
const fs = require('fs');

// 用于存储和显示语法错误的诊断集合
let diagnosticCollection = null;

/**
 * 激活扩展
 * @param {vscode.ExtensionContext} context
 */
function activate(context) {
    // 初始化诊断集合
    diagnosticCollection = vscode.languages.createDiagnosticCollection('leonbasic');
    context.subscriptions.push(diagnosticCollection);
    
    // 注册语法检测命令
    let lintDisposable = vscode.commands.registerCommand('leonbasic.lintCode', async function() {
        const editor = vscode.window.activeTextEditor;
        if (editor && editor.document.languageId === 'leonbasic') {
            lintDocument(editor.document);
        }
    });
    context.subscriptions.push(lintDisposable);
    
    // 监听文件保存事件
    let saveDisposable = vscode.workspace.onDidSaveTextDocument(document => {
        if (document.languageId === 'leonbasic') {
            lintDocument(document);
        }
    });
    context.subscriptions.push(saveDisposable);
    
    // 监听文件打开事件
    let openDisposable = vscode.workspace.onDidOpenTextDocument(document => {
        if (document.languageId === 'leonbasic') {
            lintDocument(document);
        }
    });
    context.subscriptions.push(openDisposable);
    
    // 对已打开的leonbasic文件进行语法检测
    for (const editor of vscode.window.visibleTextEditors) {
        if (editor.document.languageId === 'leonbasic') {
            lintDocument(editor.document);
        }
    }
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
 * 对LeonBasic文档进行语法检测
 * @param {vscode.TextDocument} document 要检测的文档
 */
async function lintDocument(document) {
    // 清除之前的诊断
    diagnosticCollection.clear();
    
    const diagnostics = [];
    const text = document.getText();
    const lines = text.split('\n');
    
    // 简单的语法规则检查
    checkBasicSyntaxRules(lines, diagnostics, document);
    
    // 让用户选择leonlang可执行文件进行更深入的语法检查
    const useCompilerCheck = vscode.workspace.getConfiguration('leonbasic').get('useCompilerForLinting', false);
    if (useCompilerCheck) {
        await checkWithCompiler(document, diagnostics);
    }
    
    // 设置诊断信息
    diagnosticCollection.set(document.uri, diagnostics);
}

/**
 * 基本语法规则检查
 * @param {string[]} lines 文档的行
 * @param {vscode.Diagnostic[]} diagnostics 诊断信息数组
 * @param {vscode.TextDocument} document 文档对象
 */
function checkBasicSyntaxRules(lines, diagnostics, document) {
    // 括号匹配检查
    const bracketsStack = [];
    
    for (let lineIndex = 0; lineIndex < lines.length; lineIndex++) {
        const line = lines[lineIndex];
        
        // 检查require语句格式
        if (line.trim().startsWith('require')) {
            if (!line.trim().match(/^require\("[^"]*"\);?$/)) {
                const match = line.trim().match(/^require/);
                if (match) {
                    const position = new vscode.Position(lineIndex, match.index);
                    const range = new vscode.Range(position, new vscode.Position(lineIndex, line.length));
                    diagnostics.push(new vscode.Diagnostic(
                        range,
                        'require语句格式错误，应为: require("库名");',
                        vscode.DiagnosticSeverity.Error
                    ));
                }
            }
        }
        
        // 检查basic.print函数调用格式
        if (line.includes('basic.print')) {
            // 简化的检查，实际格式可能更复杂
            if (!line.match(/basic\.print\([^)]*\);?/)) {
                const match = line.match(/basic\.print/);
                if (match) {
                    const position = new vscode.Position(lineIndex, match.index);
                    const range = new vscode.Range(position, new vscode.Position(lineIndex, match.index + 11));
                    diagnostics.push(new vscode.Diagnostic(
                        range,
                        'basic.print函数调用格式可能有错误',
                        vscode.DiagnosticSeverity.Warning
                    ));
                }
            }
        }
        
        // 检查变量定义格式
        if (line.trim().startsWith('var(')) {
            if (!line.trim().match(/^var\([a-zA-Z0-9_]+\)\s*=/)) {
                const match = line.trim().match(/^var\(/);
                if (match) {
                    const position = new vscode.Position(lineIndex, 0);
                    const range = new vscode.Range(position, new vscode.Position(lineIndex, line.length));
                    diagnostics.push(new vscode.Diagnostic(
                        range,
                        '变量定义格式错误，应为: var(变量名) = 值;',
                        vscode.DiagnosticSeverity.Error
                    ));
                }
            }
        }
        
        // 括号匹配检查
        for (let charIndex = 0; charIndex < line.length; charIndex++) {
            const char = line[charIndex];
            if (char === '(' || char === '{' || char === '[') {
                bracketsStack.push({ char, lineIndex, charIndex });
            } else if (char === ')' || char === '}' || char === ']') {
                const matching = bracketsStack.pop();
                if (!matching) {
                    // 多余的右括号
                    const position = new vscode.Position(lineIndex, charIndex);
                    const range = new vscode.Range(position, new vscode.Position(lineIndex, charIndex + 1));
                    diagnostics.push(new vscode.Diagnostic(
                        range,
                        '多余的右括号',
                        vscode.DiagnosticSeverity.Error
                    ));
                } else if (
                    (char === ')' && matching.char !== '(') ||
                    (char === '}' && matching.char !== '{') ||
                    (char === ']' && matching.char !== '[')
                ) {
                    // 括号不匹配
                    const position = new vscode.Position(lineIndex, charIndex);
                    const range = new vscode.Range(position, new vscode.Position(lineIndex, charIndex + 1));
                    diagnostics.push(new vscode.Diagnostic(
                        range,
                        `括号不匹配，期望${getMatchingBracket(matching.char)}`,
                        vscode.DiagnosticSeverity.Error
                    ));
                }
            }
        }
        
        // 检查语句结束符（简单检查，有些行可能不应该以分号结尾，如函数定义的开始行）
        const trimmedLine = line.trim();
        if (trimmedLine && !trimmedLine.endsWith(';') && !trimmedLine.endsWith('{') && 
            !trimmedLine.endsWith('}') && !trimmedLine.startsWith('//') && 
            !trimmedLine.includes('function') && !trimmedLine.includes('func(')) {
            // 排除一些不需要分号的情况
            if (!trimmedLine.startsWith('if') && !trimmedLine.startsWith('else') && 
                !trimmedLine.startsWith('for') && !trimmedLine.startsWith('while') && 
                !trimmedLine.startsWith('try') && !trimmedLine.startsWith('catch')) {
                const position = new vscode.Position(lineIndex, line.length);
                const range = new vscode.Range(position, new vscode.Position(lineIndex, line.length));
                diagnostics.push(new vscode.Diagnostic(
                    range,
                    '语句可能缺少分号',
                    vscode.DiagnosticSeverity.Warning
                ));
            }
        }
    }
    
    // 检查未闭合的括号
    for (const bracket of bracketsStack) {
        const position = new vscode.Position(bracket.lineIndex, bracket.charIndex);
        const range = new vscode.Range(position, new vscode.Position(bracket.lineIndex, bracket.charIndex + 1));
        diagnostics.push(new vscode.Diagnostic(
            range,
            `未闭合的括号${bracket.char}`,
            vscode.DiagnosticSeverity.Error
        ));
    }
}

/**
 * 获取匹配的括号
 * @param {string} bracket 括号字符
 * @returns {string} 匹配的括号
 */
function getMatchingBracket(bracket) {
    switch (bracket) {
        case '(': return ')';
        case '{': return '}';
        case '[': return ']';
        default: return '';
    }
}

/**
 * 使用编译器进行语法检查
 * @param {vscode.TextDocument} document 文档对象
 * @param {vscode.Diagnostic[]} diagnostics 诊断信息数组
 */
async function checkWithCompiler(document, diagnostics) {
    try {
        // 让用户选择leonlang可执行文件
        const options = {
            title: '选择LeonBasic可执行文件进行语法检查',
            filters: {
                '可执行文件': ['exe'],
                '所有文件': ['*']
            },
            canSelectMany: false
        };
        
        const result = await vscode.window.showOpenDialog(options);
        if (!result || result.length === 0) {
            return;
        }
        
        const leonlangPath = result[0].fsPath;
        const filePath = document.uri.fsPath;
        
        // 执行编译检查（使用--check或类似参数，如果支持的话）
        // 这里假设leonlang支持某种语法检查模式
        const command = `"${leonlangPath}" "${filePath}"`;
        
        const { stderr } = await new Promise((resolve, reject) => {
            exec(command, (error, stdout, stderr) => {
                resolve({ stdout, stderr });
            });
        });
        
        // 解析错误输出并添加到诊断
        // 注意：这里需要根据实际的错误输出格式进行调整
        const errorLines = stderr.split('\n');
        for (const errorLine of errorLines) {
            if (errorLine.includes('error') || errorLine.includes('Error')) {
                // 尝试从错误消息中提取行号和错误信息
                // 这需要根据leonlang的实际错误输出格式进行调整
                // 这里只是一个示例
                const lineMatch = errorLine.match(/line (\d+)/i);
                if (lineMatch) {
                    const lineNumber = parseInt(lineMatch[1]) - 1; // 转换为0索引
                    if (lineNumber >= 0 && lineNumber < document.lineCount) {
                        const line = document.lineAt(lineNumber);
                        const range = new vscode.Range(lineNumber, 0, lineNumber, line.text.length);
                        diagnostics.push(new vscode.Diagnostic(
                            range,
                            errorLine.trim(),
                            vscode.DiagnosticSeverity.Error
                        ));
                    }
                }
            }
        }
    } catch (err) {
        console.error('使用编译器进行语法检查时出错:', err);
    }
}

// 导出激活函数
module.exports = {
    activate,
    deactivate
};

/**
 * 停用扩展
 */
function deactivate() {
    // 清理诊断集合
    if (diagnosticCollection) {
        diagnosticCollection.clear();
        diagnosticCollection.dispose();
    }
}