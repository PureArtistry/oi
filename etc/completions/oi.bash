_oi() {
    local i cur prev opts cmds
    COMPREPLY=()
    cur="${COMP_WORDS[COMP_CWORD]}"
    prev="${COMP_WORDS[COMP_CWORD-1]}"
    cmd=""
    opts=""

    for i in ${COMP_WORDS[@]}
    do
        case "${i}" in
            "$1")
                cmd="oi"
                ;;
            *)
                ;;
        esac
    done

    case "${cmd}" in
        oi)
            opts="-h -V -a -u -q -r -s -c -L -l -p --help --version --all --urls --quiet --raw --save --cache --clean --list --lang --pick <query>..."
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 1 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --lang)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -l)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --pick)
                    COMPREPLY=($(compgen -W "basic1 basic2 clock conversions currency define holidays lists lyrics maths pronounce snippets1 snippets2 sports summary translate weather" -- "${cur}"))
                    return 0
                    ;;
                -p)
                    COMPREPLY=($(compgen -W "basic1 basic2 clock conversions currency define holidays lists lyrics maths pronounce snippets1 snippets2 sports summary translate weather" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
    esac
}

complete -F _oi -o bashdefault -o default oi
