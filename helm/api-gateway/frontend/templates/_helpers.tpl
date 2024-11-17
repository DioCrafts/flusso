# chart/templates/_helpers.tpl
{{/*
Return the name of the deployment
*/}}
{{- define "frontend.fullname" -}}
{{ .Release.Name }}-frontend
{{- end -}}

