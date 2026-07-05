
using namespace System.Management.Automation
using namespace System.Management.Automation.Language

Register-ArgumentCompleter -Native -CommandName 'richls' -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)

    $commandElements = $commandAst.CommandElements
    $command = @(
        'richls'
        for ($i = 1; $i -lt $commandElements.Count; $i++) {
            $element = $commandElements[$i]
            if ($element -isnot [StringConstantExpressionAst] -or
                $element.StringConstantType -ne [StringConstantType]::BareWord -or
                $element.Value.StartsWith('-') -or
                $element.Value -eq $wordToComplete) {
                break
        }
        $element.Value
    }) -join ';'

    $completions = @(switch ($command) {
        'richls' {
            [CompletionResult]::new('--sort', '--sort', [CompletionResultType]::ParameterName, 'Sort by name, size, or mtime')
            [CompletionResult]::new('-l', '-l', [CompletionResultType]::ParameterName, 'Show ls -l style metadata and rich information')
            [CompletionResult]::new('--long', '--long', [CompletionResultType]::ParameterName, 'Show ls -l style metadata and rich information')
            [CompletionResult]::new('-a', '-a', [CompletionResultType]::ParameterName, 'Show hidden files')
            [CompletionResult]::new('--all', '--all', [CompletionResultType]::ParameterName, 'Show hidden files')
            [CompletionResult]::new('--respect-ignore', '--respect-ignore', [CompletionResultType]::ParameterName, 'Hide entries matched by .gitignore or .dockerignore')
            [CompletionResult]::new('--complete', '--complete', [CompletionResultType]::ParameterName, 'Generate shell completion files')
            [CompletionResult]::new('--humanize', '--humanize', [CompletionResultType]::ParameterName, 'humanize')
            [CompletionResult]::new('--tagline', '--tagline', [CompletionResultType]::ParameterName, 'tagline')
            [CompletionResult]::new('--pdf-title', '--pdf-title', [CompletionResultType]::ParameterName, 'pdf-title')
            [CompletionResult]::new('--new-mark', '--new-mark', [CompletionResultType]::ParameterName, 'new-mark')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
    })

    $completions.Where{ $_.CompletionText -like "$wordToComplete*" } |
        Sort-Object -Property ListItemText
}
