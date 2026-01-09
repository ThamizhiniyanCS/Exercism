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

  $Greet = $HeyBob.TrimEnd()
  
  $IsSilence = -not $Greet.Length
  $IsQuestion = $Greet[-1] -eq '?'
  $IsYelling = $Greet.ToUpper() -ceq $Greet -and $Greet -match "[A-Za-z]"

  if ($IsSilence) {
    return "Fine. Be that way!"
  }

  if ($IsQuestion) {
    if ($IsYelling) {
      return "Calm down, I know what I'm doing!"
    }

    return "Sure."
  }

  if ($IsYelling) {
    return "Whoa, chill out!"
  }

  "Whatever."
}
