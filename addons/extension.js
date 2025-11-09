const vscode = require('vscode');
const { exec } = require('child_process');
const path = require('path');
const fs = require('fs');

// 用于存储和显示语法错误的诊断集合
let diagnosticCollection = null;
// 内存中缓存的编译器路径，确保即使配置保存失败也能在当前会话中使用
let cachedCompilerPath = null;

/**
 * 激活扩展
 * @param {vscode.ExtensionContext} context
 */
function activate(context) {
    // 初始化诊断集合
    diagnosticCollection = vscode.languages.createDiagnosticCollection('leonbasic');
    context.subscriptions.push(diagnosticCollection);
    
    // 尝试从配置中加载编译器路径到缓存
    try {
        const config = vscode.workspace.getConfiguration('leonbasic');
        const savedPath = config.get('compilerPath', '');
        if (savedPath) {
            cachedCompilerPath = savedPath;
            console.log(`已从配置加载编译器路径: ${savedPath}`);
        }
    } catch (error) {
        console.log('加载配置失败，但仍可正常使用扩展');
    }
    
    // 监听文档更改事件，实现实时语法检查
    let changeDisposable = vscode.workspace.onDidChangeTextDocument(event => {
        if (event.document.languageId === 'leonbasic') {
            // 延迟检查，避免频繁触发
            setTimeout(() => {
                lintDocument(event.document);
            }, 500);
        }
    });
    context.subscriptions.push(changeDisposable);
    
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
    // 获取编译器路径
    async function getCompilerPath() {
        // 优先使用内存缓存的路径
        if (cachedCompilerPath) {
            console.log(`使用缓存的编译器路径: ${cachedCompilerPath}`);
            return cachedCompilerPath;
        }
        
        // 如果缓存中没有，尝试从配置中获取
        let compilerPath = '';
        try {
            const config = vscode.workspace.getConfiguration('leonbasic');
            compilerPath = config.get('compilerPath', '');
        } catch (error) {
            console.log('读取配置失败，将使用选择模式');
        }
        
        // 如果没有保存的路径，让用户选择
        if (!compilerPath) {
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
                return null;
            }
            
            compilerPath = result[0].fsPath;
            // 首先更新内存缓存，确保当前会话可用
            cachedCompilerPath = compilerPath;
            
            // 尝试保存到配置中（可选，即使失败也不影响使用）
            try {
                const config = vscode.workspace.getConfiguration('leonbasic');
                // 尝试多种配置范围
                const scopes = [
                    vscode.ConfigurationTarget.WorkspaceFolder,
                    vscode.ConfigurationTarget.Workspace,
                    vscode.ConfigurationTarget.Global
                ];
                
                for (const scope of scopes) {
                    try {
                        await config.update('compilerPath', compilerPath, scope);
                        vscode.window.showInformationMessage(`已保存编译器路径: ${compilerPath}`);
                        break; // 保存成功则退出循环
                    } catch (scopeError) {
                        console.log(`使用 ${scope} 保存配置失败: ${scopeError.message}`);
                        // 继续尝试下一个范围
                    }
                }
            } catch (error) {
                // 即使配置保存失败，由于已经更新了缓存，用户仍可以在当前会话中使用
                vscode.window.showInformationMessage(`编译器路径已设置: ${compilerPath}`);
            }
        } else {
            // 将配置中的路径也缓存起来
            cachedCompilerPath = compilerPath;
        }
        
        return compilerPath;
    }
    
    // 注册更改编译器路径命令
    let changeCompilerPathDisposable = vscode.commands.registerCommand('leonbasic.changeCompilerPath', async function () {
        const options = {
            title: '选择LeonBasic可执行文件',
            filters: {
                '可执行文件': ['exe'],
                '所有文件': ['*']
            },
            canSelectMany: false
        };
        
        const result = await vscode.window.showOpenDialog(options);
        if (result && result.length > 0) {
            const compilerPath = result[0].fsPath;
            
            // 首先更新内存缓存
            cachedCompilerPath = compilerPath;
            
            // 尝试保存到配置中（可选）
            let configSaved = false;
            try {
                const config = vscode.workspace.getConfiguration('leonbasic');
                // 尝试多种配置范围
                const scopes = [
                    vscode.ConfigurationTarget.WorkspaceFolder,
                    vscode.ConfigurationTarget.Workspace,
                    vscode.ConfigurationTarget.Global
                ];
                
                for (const scope of scopes) {
                    try {
                        await config.update('compilerPath', compilerPath, scope);
                        configSaved = true;
                        break;
                    } catch (scopeError) {
                        // 继续尝试下一个范围
                    }
                }
            } catch (error) {
                // 忽略错误，因为内存缓存已经更新
            }
            
            // 根据保存结果显示不同的消息
            if (configSaved) {
                vscode.window.showInformationMessage(`已更新编译器路径: ${compilerPath}`);
            } else {
                vscode.window.showInformationMessage(`编译器路径已更新: ${compilerPath}`);
            }
        }
    });
    context.subscriptions.push(changeCompilerPathDisposable);
    
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
        
        // 获取编译器路径（从配置中读取或让用户选择）
        const leonlangPath = await getCompilerPath();
        if (!leonlangPath) {
            vscode.window.showInformationMessage('未选择LeonBasic可执行文件');
            return;
        }

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
        // 使用getCompilerPath函数获取编译器路径（会优先使用缓存）
        const leonlangPath = await getCompilerPath();
        
        if (!leonlangPath) {
            vscode.window.showInformationMessage('请先设置LeonBasic编译器路径');
            return;
        }
        
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