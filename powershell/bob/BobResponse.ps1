Function Get-BobResponse() {
    <#
    .SYNOPSIS
    Bob is a lackadaisical teenager. In conversation, his responses are very limited.
    
    .DESCRIPTION
    Bob is a lackadaisical teenager. In conversation, his responses are very limited.

    Bob answers 'Sure.' if you ask him a question.

    He answers 'Whoa, chill out!' if you yell at him.

    He answers 'Calm down, I know what I'm doing!' if you yell a question at him.

    He says 'Fine. Be that way!' if you address him without actually saying
    anything.

    He answers 'Whatever.' to anything else.
    
    .PARAMETER HeyBob
    The sentence you say to Bob.
    
    .EXAMPLE
    Get-BobResponse -HeyBob "Hi Bob"
    #>
    [CmdletBinding()]
    Param(
        [string]$HeyBob
    )

    if ($HeyBob -cmatch '^[A-Z\s]+\?$') {          
      Write-Output "Calm down, I know what I'm doing!"
    } elseif ($HeyBob -cmatch '^[^a-z]+$' -and $HeyBob -cmatch '[A-Z]') {         
      Write-Output "Whoa, chill out!"                  
    } elseif ($HeyBob -match '\?\s*$') {                       
      Write-Output "Sure."
    } elseif ($HeyBob -match '^\s*$') {                
      Write-Output "Fine. Be that way!"                
    } else {                                           
      Write-Output "Whatever."                         
    }                                                                                             
}
