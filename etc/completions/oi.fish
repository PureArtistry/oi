complete -c oi -s l -l lang -d 'Specify the language to use (eg: en-GB)' -r -f
complete -c oi -s p -l pick -d 'Target specific answers, use -- to stop parsing arguments' -r -f -a "{basic1	,basic2	,clock	,conversions	,currency	,define	,holidays	,lists	,lyrics	,maths	,pronounce	,snippets1	,snippets2	,sports	,summary	,translate	,weather	}"
complete -c oi -s h -l help -d 'Print help information'
complete -c oi -s V -l version -d 'Print version information'
complete -c oi -s a -l all -d 'Prints all of the answers found'
complete -c oi -s u -l urls -d 'Also print a list of the top urls associated with your query'
complete -c oi -s q -l quiet -d 'Only print the answer (if applicable) and error messages'
complete -c oi -s r -l raw -d 'Raw output (use --help for details)'
complete -c oi -s s -l save -d 'Saves the raw HTML for this query'
complete -c oi -s c -l cache -d 'Use the most recent cached HTML'
complete -c oi -l clean -d 'Remove all previously saved results'
complete -c oi -s L -l list -d 'Prints a table of all the valid answer selectors'
