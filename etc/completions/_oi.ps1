
using namespace System.Management.Automation
using namespace System.Management.Automation.Language

Register-ArgumentCompleter -Native -CommandName 'oi' -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)

    $commandElements = $commandAst.CommandElements
    $command = @(
        'oi'
        for ($i = 1; $i -lt $commandElements.Count; $i++) {
            $element = $commandElements[$i]
            if ($element -isnot [StringConstantExpressionAst] -or
                $element.StringConstantType -ne [StringConstantType]::BareWord -or
                $element.Value.StartsWith('-')) {
                break
        }
        $element.Value
    }) -join ';'

    $completions = @(switch ($command) {
        'oi' {
            [CompletionResult]::new('-l', 'l', [CompletionResultType]::ParameterName, 'Specify the language to use (eg: en-GB)')
            [CompletionResult]::new('--lang', 'lang', [CompletionResultType]::ParameterName, 'Specify the language to use (eg: en-GB)')
            [CompletionResult]::new('-p', 'p', [CompletionResultType]::ParameterName, 'Target specific answers, use -- to stop parsing arguments')
            [CompletionResult]::new('--pick', 'pick', [CompletionResultType]::ParameterName, 'Target specific answers, use -- to stop parsing arguments')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Print version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Print version information')
            [CompletionResult]::new('-a', 'a', [CompletionResultType]::ParameterName, 'Prints all of the answers found')
            [CompletionResult]::new('--all', 'all', [CompletionResultType]::ParameterName, 'Prints all of the answers found')
            [CompletionResult]::new('-u', 'u', [CompletionResultType]::ParameterName, 'Also print a list of the top urls associated with your query')
            [CompletionResult]::new('--urls', 'urls', [CompletionResultType]::ParameterName, 'Also print a list of the top urls associated with your query')
            [CompletionResult]::new('-q', 'q', [CompletionResultType]::ParameterName, 'Only print the answer (if applicable) and error messages')
            [CompletionResult]::new('--quiet', 'quiet', [CompletionResultType]::ParameterName, 'Only print the answer (if applicable) and error messages')
            [CompletionResult]::new('-r', 'r', [CompletionResultType]::ParameterName, 'Raw output (use --help for details)')
            [CompletionResult]::new('--raw', 'raw', [CompletionResultType]::ParameterName, 'Raw output (use --help for details)')
            [CompletionResult]::new('-s', 's', [CompletionResultType]::ParameterName, 'Saves the raw HTML for this query')
            [CompletionResult]::new('--save', 'save', [CompletionResultType]::ParameterName, 'Saves the raw HTML for this query')
            [CompletionResult]::new('-c', 'c', [CompletionResultType]::ParameterName, 'Use the most recent cached HTML')
            [CompletionResult]::new('--cache', 'cache', [CompletionResultType]::ParameterName, 'Use the most recent cached HTML')
            [CompletionResult]::new('--clean', 'clean', [CompletionResultType]::ParameterName, 'Remove all previously saved results')
            [CompletionResult]::new('-L', 'L', [CompletionResultType]::ParameterName, 'Prints a table of all the valid answer selectors')
            [CompletionResult]::new('--list', 'list', [CompletionResultType]::ParameterName, 'Prints a table of all the valid answer selectors')
            break
        }
    })

    $completions.Where{ $_.CompletionText -like "$wordToComplete*" } |
        Sort-Object -Property ListItemText
}
